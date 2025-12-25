use actix_web::{HttpResponse, Responder, get, post, web};
use common::{
    accounts::model::{
        AccountRecord,
        dto::{AccountInfo, CreateAccountRequest},
    },
    tables::ACCOUNTS,
};
use serde::Deserialize;
use serde_json::json;
use surrealdb::{Surreal, engine::remote::ws::Client};
use utoipa::ToSchema;

pub struct AppState {
    pub db: Surreal<Client>,
}

#[derive(Deserialize, ToSchema)]
pub struct DepositRequest {
    #[schema(example = 100.0, minimum = 0.01)]
    pub amount: f64,
}

/// Create a new account.
#[utoipa::path(
    responses(
        (
            status = 201,
            description = "Account created successfully",
            body = AccountInfo,
            example = json!(AccountInfo {
                id: "zi1yqmaesl1qdlhbmwjr".to_string(),
                balance: 0.0
            })
        ),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/accounts")]
pub async fn create_account(
    data: web::Data<AppState>,
    req: web::Json<CreateAccountRequest>,
) -> impl Responder {
    let request_data = req.into_inner();

    let created: Result<Option<AccountRecord>, _> =
        data.db.create(ACCOUNTS).content(request_data).await;

    match created {
        Ok(Some(record)) => HttpResponse::Created().json(AccountInfo::from(record).id),
        Ok(None) => HttpResponse::InternalServerError().body("Failed to create account"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get account balance by ID.
#[utoipa::path(
    responses(
        (
            status = 200,
            description = "Account found",
            body = AccountInfo,
            example = json!(AccountInfo {
                id: "zi1yqmaesl1qdlhbmwjr".to_string(),
                balance: 150.0
            })
        ),
        (status = 404, description = "Account not found"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("id", description = "Account ID")
    )
)]
#[get("/accounts/{id}")]
pub async fn get_balance(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let result: Result<Option<AccountRecord>, _> = data.db.select((ACCOUNTS, id)).await;

    match result {
        Ok(Some(record)) => HttpResponse::Ok().json(AccountInfo::from(record)),
        Ok(None) => HttpResponse::NotFound().body("Account not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Deposit money to account.
/// The amount of money to deposit must be positive.
#[utoipa::path(
    responses(
        (
            status = 200,
            description = "Deposit successful",
            body = AccountInfo,
            example = json!(AccountInfo {
                id: "zi1yqmaesl1qdlhbmwjr".to_string(),
                balance: 200.0
            })
        ),
        (status = 400, description = "Invalid amount"),
        (status = 404, description = "Account not found"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("id", description = "Account ID")
    )
)]
#[post("/accounts/{id}/deposit")]
pub async fn deposit(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<DepositRequest>,
) -> impl Responder {
    if req.amount <= 0.0 {
        return HttpResponse::BadRequest().body("Amount must be positive");
    }

    let id = path.into_inner();
    let sql = "UPDATE type::thing($table, $id) SET balance += $amount RETURN AFTER";

    let mut response = data
        .db
        .query(sql)
        .bind(("table", ACCOUNTS))
        .bind(("id", id))
        .bind(("amount", req.amount))
        .await;

    match response {
        Ok(mut res) => {
            let updated: Result<Option<AccountRecord>, _> = res.take(0);
            match updated {
                Ok(Some(record)) => HttpResponse::Ok().json(AccountInfo::from(record)),
                Ok(None) => HttpResponse::NotFound().body("Account not found"),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
