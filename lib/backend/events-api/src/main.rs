use api::get_event::get_event;
use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lambda_http::{run, Error};
use serde_json::{json, Value};
use tracing_subscriber::fmt::format;

mod api;
mod configuration;
mod images;
mod model;

fn setup_logging() {
    tracing_subscriber::fmt()
        .event_format(format::json().with_thread_ids(false).with_ansi(false))
        .init();
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

async fn real_main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let api = Router::new()
        .route("/event/{eventId}", get(get_event))
        .route("/event/{eventId}/images", post(api::post_image::post_image))
        // 10 mb limit for images
        .layer(DefaultBodyLimit::disable())
        .route("/user_error", get(user_error))
        .route("/server_error", get(server_error))
        .with_state(dynamodb_client);

    let app = Router::new().nest("/api", api);

    run(app).await
}

fn main() -> Result<(), Error> {
    setup_logging();
    let _guard = sentry::init(sentry::ClientOptions {
        attach_stacktrace: true,
        ..Default::default()
    });

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(real_main())
}
