use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountInfo {
    #[schema(example = "zi1yqmaesl1qdlhbmwjr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    
    #[schema(example = "1000.0")]
    balance: f64,
}
