use axum::{routing::get, Json, Router};
use lambda_http::{run, Error};
use serde_json::{json, Value};

async fn get_event() -> Json<Value> {
    Json(json!({"msg": "hello"}))
}

async fn panics() -> Json<Value> {
    panic!("bad again!");
}

fn main() -> Result<(), Error> {
    let _guard = sentry::init(());
    let app = Router::new()
        .route("/", get(get_event))
        .route("/panic", get(panics))
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction());

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run(app))
}
