use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use super::{OrderRecord, OrderStatus};

/// Request order creation
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrderRequest {
    #[schema(example = "zi1yqmaesl1qdlhbmwjr")]
    pub user_id: String,
    
    #[schema(example = 1500.0, minimum = 0.01)]
    pub amount: f64,
    
    #[schema(example = "Order for laptop")]
    pub description: String,
}

/// Response with order information
#[derive(Debug, Serialize, ToSchema)]
pub struct OrderResponse {
    #[schema(example = "zi1yqmaesl1qdlhbmwjr")]
    pub id: String,
    
    #[schema(example = "zi1yqmaesl1qdlhbmwjr")]
    pub user_id: String,
    
    #[schema(example = 1500.0)]
    pub amount: f64,
    
    #[schema(example = "Order for laptop")]
    pub description: String,
    
    #[schema(example = "New")]
    pub status: OrderStatus,
}

/// Конвертация из записи БД в публичный ответ
impl From<OrderRecord> for OrderResponse {
    fn from(record: OrderRecord) -> Self {
        Self {
            id: record.id
                .map(|t| t.id.to_string())
                .unwrap_or_default(),
            user_id: record.user_id,
            amount: record.amount,
            description: record.description,
            status: record.status,
        }
    }
}