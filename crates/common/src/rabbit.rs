use lapin::{
    Channel, ExchangeKind,
    options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
};

// Константы для RabbitMQ (Exchange и Routing Keys)
pub const EXCHANGE_ORDER: &str = "order_exchange";

pub const ROUTING_KEY_ORDER_CREATED: &str = "order.created";
pub const ROUTING_KEY_ORDER_PAID: &str = "order.paid";

pub const QUEUE_PAYMENTS: &str = "q_payments";
pub const QUEUE_ORDERS: &str = "q_orders";

pub async fn setup_rabbit(channel: &Channel) -> lapin::Result<()> {
    channel
        .exchange_declare(
            EXCHANGE_ORDER,
            ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            QUEUE_PAYMENTS,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            QUEUE_PAYMENTS,
            EXCHANGE_ORDER,
            ROUTING_KEY_ORDER_CREATED,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            QUEUE_ORDERS,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            QUEUE_ORDERS,
            EXCHANGE_ORDER,
            ROUTING_KEY_ORDER_PAID,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(())
}
