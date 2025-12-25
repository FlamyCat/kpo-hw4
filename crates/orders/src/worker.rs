use crate::model::OrderStatus;
use common::{events::PaymentProcessedEvent, rabbit::QUEUE_ORDERS, tables::ORDERS};
use futures::StreamExt;
use lapin::{
    Channel,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub async fn start_orders_consumer(db: Surreal<Client>, channel: Channel) {
    println!("Starting Orders Consumer...");

    let mut consumer = channel
        .basic_consume(
            QUEUE_ORDERS,
            "orders_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to create consumer");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let payload = delivery.data.clone();

            let event: PaymentProcessedEvent = match serde_json::from_slice(&payload) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Error deserializing: {}", e);
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                    continue;
                }
            };

            println!(
                "Updating Order {}: success={}",
                event.order_id, event.success
            );

            let new_status = if event.success {
                OrderStatus::Finished
            } else {
                OrderStatus::Cancelled
            };

            let _ = db
                .update::<Option<serde::de::IgnoredAny>>((ORDERS, &event.order_id))
                .merge(serde_json::json!({
                    "status": new_status
                }))
                .await;

            let _ = delivery.ack(BasicAckOptions::default()).await;
        }
    }
}
