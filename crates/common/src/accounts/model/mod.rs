pub mod dto;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountInfo {
    #[schema(example = "zi1yqmaesl1qdlhbmwjr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[schema(example = "1000.0")]
    pub balance: f64,
}
