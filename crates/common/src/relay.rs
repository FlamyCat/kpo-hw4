use crate::db_utils::OutboxRecord;
use crate::tables::OUTBOX;
use futures::StreamExt;
use lapin::{BasicProperties, Channel, options::BasicPublishOptions};
use surrealdb::{Action, Surreal, engine::remote::ws::Client};

/// Запускает процесс пересылки сообщений из Outbox в RabbitMQ
pub async fn start_outbox_relay(db: Surreal<Client>, channel: Channel) {
    eprintln!("Starting Outbox Relay");

    let pending: Result<Vec<OutboxRecord>, _> = db
        .query("SELECT * FROM outbox WHERE processed = false")
        .await
        .map(|mut r| r.take(0).unwrap_or_default());

    if let Ok(records) = pending {
        for record in records {
            process_record(&db, &channel, record).await;
        }
    }

    let mut stream = match db.select(OUTBOX).live().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to subscribe to outbox: {}", e);
            return;
        }
    };

    eprintln!("Listening for new outbox messages");

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(notification) => {
                println!("DEBUG: Got notification action: {:?}", notification.action); // <--- DEBUG

                if notification.action == Action::Create {
                    println!("DEBUG: Raw data: {:?}", notification.data); // <--- DEBUG

                    match serde_json::from_value::<OutboxRecord>(notification.data) {
                        Ok(record) => {
                            println!("DEBUG: Sending record to Rabbit: {:?}", record.id); // <--- DEBUG
                            process_record(&db, &channel, record).await;
                        }
                        Err(e) => {
                            eprintln!("CRITICAL: Failed to deserialize outbox record: {}", e); // <--- DEBUG
                        }
                    }
                }
            }
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }
}

async fn process_record(db: &Surreal<Client>, channel: &Channel, record: OutboxRecord) {
    let payload = record.payload.as_bytes();

    let publish_res = channel
        .basic_publish(
            &record.exchange,
            &record.routing_key,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await;

    match publish_res {
        Ok(_) => {
            if let Some(id) = record.id {
                let _ = db
                    .delete::<Option<OutboxRecord>>((id.tb, id.id.to_string()))
                    .await;
            }
        }
        Err(e) => {
            eprintln!("Failed to publish message: {}", e);
        }
    }
}
