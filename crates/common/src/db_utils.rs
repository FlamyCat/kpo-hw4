use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Структура записи в таблице Outbox
#[derive(Debug, Serialize, Deserialize)]
pub struct OutboxRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub payload: String,
    pub exchange: String,
    pub routing_key: String,
    pub created_at: String,
    pub processed: bool,
}
