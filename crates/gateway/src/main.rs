use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use awc::Client;
use url::Url;
use utoipa_swagger_ui::{Config, SwaggerUi, Url as SwaggerUrl};

#[derive(Clone)]
struct ConfigData {
    orders_url: String,
    payments_url: String,
}

async fn forward_request(
    req: HttpRequest,
    payload: web::Payload,
    target_url: web::Data<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let mut new_url = Url::parse(&target_url).unwrap();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();

    let res = forwarded_req
        .send_stream(payload)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut client_resp = HttpResponse::build(res.status());
    for (header_name, header_value) in res.headers().iter() {
        client_resp.insert_header((header_name.clone(), header_value.clone()));
    }

    Ok(client_resp.streaming(res))
}

async fn fetch_openapi_spec(
    target_url: web::Data<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let spec_url = format!("{}/api-docs/openapi.json", target_url.as_str());

    let mut res = client
        .get(&spec_url)
        .send()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let body = res
        .body()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orders_url =
        std::env::var("ORDERS_SERVICE_URL").unwrap_or_else(|_| "http://127.0.0.1:8082".to_string());

    let payments_url = std::env::var("PAYMENTS_SERVICE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8081".to_string());

    println!("Gateway running at http://0.0.0.0:8080");
    println!("Swagger UI available at http://0.0.0.0:8080/swagger-ui/");

    println!("DEBUG: Gateway configuration:");
    println!("  -> Orders Service:   {}", orders_url);
    println!("  -> Payments Service: {}", payments_url);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(Client::default()))
            .app_data(web::Data::new(ConfigData {
                orders_url: orders_url.clone(),
                payments_url: payments_url.clone(),
            }))
            .service(
                web::scope("/orders")
                    .app_data(web::Data::new(orders_url.clone()))
                    .default_service(web::to(forward_request)),
            )
            .service(
                web::scope("/accounts")
                    .app_data(web::Data::new(payments_url.clone()))
                    .default_service(web::to(forward_request)),
            )
            .route(
                "/docs/orders/openapi.json",
                web::get().to({
                    let url = orders_url.clone();
                    move |client| fetch_openapi_spec(web::Data::new(url.clone()), client)
                }),
            )
            .route(
                "/docs/payments/openapi.json",
                web::get().to({
                    let url = payments_url.clone();
                    move |client| fetch_openapi_spec(web::Data::new(url.clone()), client)
                }),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").config(Config::new(vec![
                    SwaggerUrl::new("Orders Service", "/docs/orders/openapi.json"),
                    SwaggerUrl::new("Payments Service", "/docs/payments/openapi.json"),
                ])),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
