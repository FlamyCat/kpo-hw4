use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Публичный DTO для ответов API.
/// Содержит чистый ID и только нужные поля.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountInfo {
    #[schema(example = "zi1yqmaesl1qdlhbmwjr")]
    pub id: String,

    #[schema(example = 1000.0)]
    pub balance: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    #[schema(example = 0.0, minimum = 0.0)]
    pub balance: f64,
}
