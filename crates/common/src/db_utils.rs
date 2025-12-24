use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

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

/// Подключиться к СУБД
///
/// Автоматически определяет адрес подключения:
/// * Если задана переменная окружения `IS_DOCKER=true`, используется "surrealdb:8000"
/// * Иначе используется "0.0.0.0:8000"
pub async fn connect(db: &Surreal<Client>) -> surrealdb::Result<()> {
    // Проверяем наличие и значение переменной окружения
    let is_docker = env::var("IS_DOCKER")
        .map(|val| val == "true" || val == "1")
        .unwrap_or(false);

    let address = if is_docker {
        "surrealdb:8000"
    } else {
        "0.0.0.0:8000"
    };

    db.connect::<Ws>(address).await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("kpo").use_db("kpo").await?;

    Ok(())
}
