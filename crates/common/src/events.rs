use serde::{Deserialize, Serialize};

/// Событие: Заказ создан.
/// Отправляет Orders -> Payments
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: String,
    pub user_id: String, // ID счета/пользователя
    pub amount: f64,     // Сумма к списанию
}

/// Событие: Результат оплаты.
/// Отправляет Payments -> Orders
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProcessedEvent {
    pub order_id: String,
    pub success: bool,
    // Если success = false, здесь может быть причина (например, "Insufficient funds")
    pub reason: Option<String>,
}
