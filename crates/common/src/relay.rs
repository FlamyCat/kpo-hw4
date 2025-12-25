use std::time::Duration;

use crate::db_utils::OutboxRecord;
use lapin::{BasicProperties, Channel, options::BasicPublishOptions};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Id};

/// Запускает процесс пересылки сообщений из Outbox в RabbitMQ
pub async fn start_outbox_relay(db: Surreal<Client>, channel: Channel) {
    println!("Starting Outbox Relay (Polling Mode)...");

    loop {
        let records: Result<Vec<OutboxRecord>, _> = db
            .query("SELECT * FROM outbox WHERE processed = false LIMIT 50")
            .await
            .map(|mut r| r.take(0).unwrap_or_default());

        match records {
            Ok(batch) if !batch.is_empty() => {
                println!("DEBUG: Processing {} records", batch.len());
                for record in batch {
                    process_record(&db, &channel, record).await;
                }
            }
            Ok(_) => {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => {
                eprintln!("DB Error: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn process_record(db: &Surreal<Client>, channel: &Channel, record: OutboxRecord) {
    let payload = record.payload.as_bytes();

    let msg_id = record
        .id
        .as_ref()
        .map(|t| t.id.to_string())
        .unwrap_or_else(|| Id::rand().to_string());

    // Создаем свойства сообщения
    let props = BasicProperties::default().with_message_id(msg_id.into());

    let publish_res = channel
        .basic_publish(
            &record.exchange,
            &record.routing_key,
            BasicPublishOptions::default(),
            payload,
            props,
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
