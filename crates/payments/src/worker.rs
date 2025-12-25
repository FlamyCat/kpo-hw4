use common::{
    events::{OrderCreatedEvent, PaymentProcessedEvent},
    rabbit::{EXCHANGE_ORDER, QUEUE_PAYMENTS, ROUTING_KEY_ORDER_PAID},
    tables::{ACCOUNTS, INBOX, OUTBOX},
};
use futures::StreamExt;
use lapin::{
    Channel,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub async fn start_payments_consumer(db: Surreal<Client>, channel: Channel) {
    println!("Starting Payments Consumer...");

    let mut consumer = channel
        .basic_consume(
            QUEUE_PAYMENTS,
            "payments_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to create consumer");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let payload = delivery.data.clone();

            let event: OrderCreatedEvent = match serde_json::from_slice(&payload) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Error deserializing event: {}", e);
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                    continue;
                }
            };

            println!("Processing payment for Order: {}", event.order_id);

            let msg_id = delivery
                .properties
                .message_id()
                .clone()
                .unwrap_or_else(|| "unknown".to_string().into());

            let success_event = PaymentProcessedEvent {
                order_id: event.order_id.clone(),
                success: true,
                reason: None,
            };

            let fail_account_event = PaymentProcessedEvent {
                order_id: event.order_id.clone(),
                success: false,
                reason: Some("Account not found".to_string()),
            };

            let fail_funds_event = PaymentProcessedEvent {
                order_id: event.order_id.clone(),
                success: false,
                reason: Some("Insufficient funds".to_string()),
            };

            let success_json = serde_json::to_string(&success_event).unwrap();
            let fail_account_json = serde_json::to_string(&fail_account_event).unwrap();
            let fail_funds_json = serde_json::to_string(&fail_funds_event).unwrap();

            println!(
                "DEBUG: Payments Consumer received order: {}",
                event.order_id
            );

            let sql = r#"
                BEGIN TRANSACTION;

                LET $inbox_exists = SELECT * FROM type::thing($table_inbox, $msg_id);
                
                -- Если запись есть, прерываем выполнение ошибкой.
                -- Ошибка автоматически отменит транзакцию.
                IF $inbox_exists != [] THEN
                    THROW "Duplicate message";
                END;

                -- Записываем в Inbox
                CREATE type::thing($table_inbox, $msg_id) CONTENT { processed_at: time::now() };

                -- 2. Проверка Аккаунта
                LET $account = SELECT * FROM type::thing($table_accounts, $user_id);
                
                IF $account = [] THEN
                    -- Аккаунта нет. Пишем Fail.
                    CREATE type::table($table_outbox) CONTENT {
                        payload: $fail_account_payload,
                        exchange: $exchange,
                        routing_key: $routing_key,
                        created_at: time::now(),
                        processed: false
                    };
                ELSE
                    -- Аккаунт есть. Проверяем баланс.
                    -- Внимание: доступ к массиву через [0]
                    LET $balance = $account[0].balance;
                    
                    IF $balance < $amount THEN
                        -- Денег нет. Пишем Fail.
                        CREATE type::table($table_outbox) CONTENT {
                            payload: $fail_funds_payload,
                            exchange: $exchange,
                            routing_key: $routing_key,
                            created_at: time::now(),
                            processed: false
                        };
                    ELSE
                        -- Все ок. Списываем.
                        UPDATE type::thing($table_accounts, $user_id) SET balance -= $amount;
                        
                        -- Пишем Success в Outbox.
                        CREATE type::table($table_outbox) CONTENT {
                            payload: $success_payload,
                            exchange: $exchange,
                            routing_key: $routing_key,
                            created_at: time::now(),
                            processed: false
                        };
                    END;
                END;

                COMMIT TRANSACTION;
            "#;

            let res = db
                .query(sql)
                .bind(("table_inbox", INBOX))
                .bind(("msg_id", msg_id.clone()))
                .bind(("table_accounts", ACCOUNTS))
                .bind(("user_id", event.user_id))
                .bind(("amount", event.amount))
                .bind(("table_outbox", OUTBOX))
                .bind(("success_payload", success_json))
                .bind(("fail_account_payload", fail_account_json))
                .bind(("fail_funds_payload", fail_funds_json))
                .bind(("exchange", EXCHANGE_ORDER))
                .bind(("routing_key", ROUTING_KEY_ORDER_PAID))
                .await;

            if let Err(e) = res {
                let err_string = e.to_string();
                if err_string.contains("Duplicate message") {
                    println!("DEBUG: Message {} already processed (Idempotency check)", msg_id);
                } else {
                    eprintln!("CRITICAL: Payments Transaction failed: {}", e);
                }
            } else {
                 println!("DEBUG: Payments Transaction OK");
            }

            let _ = delivery.ack(BasicAckOptions::default()).await;
        }
    }
}
