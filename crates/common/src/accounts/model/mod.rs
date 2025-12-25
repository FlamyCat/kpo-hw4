pub mod dto;

use dto::AccountInfo;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Внутренняя модель для работы с базой данных.
/// Клиент API эту структуру никогда не видит.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub balance: f64,
}

/// Конвертация из записи БД в публичный DTO
impl From<AccountRecord> for AccountInfo {
    fn from(record: AccountRecord) -> Self {
        Self {
            id: record.id.map(|t| t.id.to_string()).unwrap_or_default(),
            balance: record.balance,
        }
    }
}
