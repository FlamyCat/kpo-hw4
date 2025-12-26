use crate::model::{
    OrderRecord, OrderStatus,
    dto::{CreateOrderRequest, OrderResponse},
};
use actix_web::{HttpResponse, Responder, get, post, web};
use chrono::Utc;
use common::{
    events::OrderCreatedEvent,
    rabbit::{EXCHANGE_ORDER, ROUTING_KEY_ORDER_CREATED},
    tables::{ORDERS, OUTBOX},
};
use serde::Deserialize;
use surrealdb::{Surreal, engine::remote::ws::Client};
use utoipa::IntoParams;

pub struct AppState {
    pub db: Surreal<Client>,
}

#[derive(Deserialize, IntoParams)]
pub struct ListOrdersParams {
    /// Filter orders by User ID
    pub user_id: Option<String>,
}

/// Create a new order.
#[utoipa::path(
    path = "/orders",
    responses(
        (
            status = 201,
            description = "Order created and payment initiated",
            body = OrderResponse,
            example = json!(OrderResponse {
                id: "zi1yqmaesl1qdlhbmwjr".to_string(),
                user_id: "zi1yqmaesl1qdlhbmwjr".to_string(),
                amount: 1500.0,
                description: "Laptop".to_string(),
                status: OrderStatus::New
            })
        ),
        (status = 400, description = "Invalid data"),
        (status = 404, description = "Account not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/orders")]
pub async fn create_order(
    data: web::Data<AppState>,
    req: web::Json<CreateOrderRequest>,
) -> impl Responder {
    let req = req.into_inner();

    if req.amount <= 0.0 {
        return HttpResponse::BadRequest().body("Amount must be positive");
    }

    let order_id_key = surrealdb::sql::Id::rand();
    let order_id_thing = surrealdb::sql::Thing::from((ORDERS, order_id_key));
    let order_id_str = order_id_thing.id.to_string();

    let event = OrderCreatedEvent {
        order_id: order_id_str.clone(),
        user_id: req.user_id.clone(),
        amount: req.amount,
    };

    let payload_json = match serde_json::to_string(&event) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let sql_transaction = r#"
        BEGIN TRANSACTION;

        CREATE type::thing($table_orders, $order_id) CONTENT {
            user_id: $user_id,
            amount: $amount,
            description: $description,
            status: $status_new
        };

        CREATE type::table($table_outbox) CONTENT {
            payload: $payload,
            exchange: $exchange,
            routing_key: $routing_key,
            created_at: $created_at,
            processed: false
        };

        COMMIT TRANSACTION;
    "#;

    let res = data
        .db
        .query(sql_transaction)
        .bind(("table_orders", ORDERS))
        .bind(("order_id", order_id_str.clone()))
        .bind(("user_id", req.user_id.clone()))
        .bind(("amount", req.amount))
        .bind(("description", req.description.clone()))
        .bind(("status_new", OrderStatus::New))
        .bind(("table_outbox", OUTBOX))
        .bind(("payload", payload_json))
        .bind(("exchange", EXCHANGE_ORDER))
        .bind(("routing_key", ROUTING_KEY_ORDER_CREATED))
        .bind(("created_at", Utc::now().to_rfc3339()))
        .await;

    match res {
        Ok(_) => {
            let response = OrderResponse {
                id: order_id_str,
                user_id: req.user_id,
                amount: req.amount,
                description: req.description,
                status: OrderStatus::New,
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    params(
        ListOrdersParams
    ),
    responses(
        (
            status = 200,
            description = "List of orders",
            body = Vec<OrderResponse>
        ),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/orders")]
pub async fn list_orders(
    data: web::Data<AppState>,
    query: web::Query<ListOrdersParams>,
) -> impl Responder {
    let params = query.into_inner();

    let result: Result<Vec<OrderRecord>, _> = if let Some(uid) = params.user_id {
        data.db
            .query("SELECT * FROM orders WHERE user_id = $uid")
            .bind(("uid", uid))
            .await
            .map(|mut r| r.take(0).unwrap_or_default())
    } else {
        data.db.select(ORDERS).await
    };

    match result {
        Ok(records) => {
            let responses: Vec<OrderResponse> =
                records.into_iter().map(OrderResponse::from).collect();
            HttpResponse::Ok().json(responses)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    path = "/orders/{id}",
    responses(
        (
            status = 200,
            description = "Order details",
            body = OrderResponse
        ),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("id", description = "Order ID")
    )
)]
#[get("/orders/{id}")]
pub async fn get_order(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let result: Result<Option<OrderRecord>, _> = data.db.select((ORDERS, id)).await;

    match result {
        Ok(Some(record)) => HttpResponse::Ok().json(OrderResponse::from(record)),
        Ok(None) => HttpResponse::NotFound().body("Order not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
