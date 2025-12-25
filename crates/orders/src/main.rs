use actix_web::{App, HttpServer, web};
use common::db_utils;
use surrealdb::Surreal;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod model;
mod worker;

use model::{
    OrderStatus,
    dto::{CreateOrderRequest, OrderResponse},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        api::create_order,
        api::list_orders,
        api::get_order
    ),
    components(
        schemas(CreateOrderRequest, OrderResponse, OrderStatus)
    ),
    tags(
        (name = "orders", description = "Orders management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Surreal::init();

    db_utils::connect(&db)
        .await
        .expect("Failed to connect to SurrealDB");

    let app_state = web::Data::new(api::AppState { db });

    println!("Orders service running at http://localhost:8082");
    println!("Swagger UI available at http://localhost:8082/swagger-ui/");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(api::create_order)
            .service(api::list_orders)
            .service(api::get_order)
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await
}
