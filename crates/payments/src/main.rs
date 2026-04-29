use actix_web::{App, HttpServer, web};
use common::{
    accounts::model::dto::{AccountInfo, CreateAccountRequest},
    db_utils, rabbit,
};
use lapin::Connection;
use lapin::ConnectionProperties;
use surrealdb::Surreal;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod worker;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::create_account,
        api::get_balance,
        api::deposit
    ),
    components(
        schemas(AccountInfo, CreateAccountRequest, api::DepositRequest)
    ),
    tags(
        (name = "payments", description = "Payments management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Surreal::init();

    db_utils::connect(&db)
        .await
        .expect("Failed to connect to SurrealDB");

    let rabbit_addr =
        std::env::var("RABBITMQ_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let connection = Connection::connect(&rabbit_addr, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create channel");

    rabbit::setup_rabbit(&channel)
        .await
        .expect("Failed to setup RabbitMQ");

    let db_relay = db.clone();
    let channel_relay = connection.create_channel().await.expect("ch");
    tokio::spawn(async move {
        common::relay::start_outbox_relay(db_relay, channel_relay).await;
    });

    let db_consumer = db.clone();
    let channel_consumer = connection.create_channel().await.expect("ch");
    tokio::spawn(async move {
        worker::start_payments_consumer(db_consumer, channel_consumer).await;
    });

    let app_state = web::Data::new(api::AppState { db });

    println!("Payments service running at http://localhost:8081");
    println!("Swagger UI available at http://localhost:8081/swagger-ui/");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(api::create_account)
            .service(api::get_balance)
            .service(api::deposit)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
