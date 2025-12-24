use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    #[schema(example = 0.0, minimum = 0.0)]
    pub balance: f64,
}
