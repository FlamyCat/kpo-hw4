use actix_web::{web, App, HttpServer};
use common::{
    accounts::model::dto::{CreateAccountRequest, AccountInfo},
    db_utils,
};
use surrealdb::Surreal;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;

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
