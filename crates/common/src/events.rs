use serde::{Deserialize, Serialize};

/// Событие: Заказ создан. Отправляет Orders -> Payments
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: String, // ID заказа
    pub user_id: String,  // ID пользователя
    pub total_price: f64, // Сумма к оплате
}

/// Событие: Результат оплаты. Отправляет Payments -> Orders
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProcessedEvent {
    pub order_id: String,
    pub success: bool,
    pub reason: Option<String>,
}
