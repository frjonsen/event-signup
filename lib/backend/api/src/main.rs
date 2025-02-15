use axum::{http::StatusCode, routing::get, Json, Router};
use lambda_http::{run, Error};
use serde_json::{json, Value};

async fn get_event() -> Json<Value> {
    Json(json!({"msg": "hello"}))
}

async fn user_error() -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Err((StatusCode::BAD_REQUEST, Json(json!({"msg": "user bad!"}))))
}

async fn server_error() -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"msg": "bad!"})),
    ))
}

fn main() -> Result<(), Error> {
    let _guard = sentry::init(sentry::ClientOptions {
        attach_stacktrace: true,
        ..Default::default()
    });
    let api = Router::new()
        .route("/", get(get_event))
        .route("/user_error", get(user_error))
        .route("/server_error", get(server_error))
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction());

    let app = Router::new().nest("/api", api);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run(app))
}
