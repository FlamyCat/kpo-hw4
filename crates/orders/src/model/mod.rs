use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

pub mod dto;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, ToSchema)]
pub enum OrderStatus {
    /// The order is new and has not been processed yet.
    New,
    
    /// The order was placed successfully.
    Finished,
    
    /// There was an error processing the order.
    Cancelled,
}

/// Внутренняя модель заказа в БД (таблица `orders`)
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRecord {
    /// ID заказа
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    
    /// ID пользователя, который создал заказ
    pub user_id: String,
    
    /// Сумма заказа
    pub amount: f64,
    
    /// Описание заказа
    pub description: String,
    
    /// Статус заказа
    pub status: OrderStatus,
}