use std::{env, sync::LazyLock};

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use lambda_http::{run, Error};
use serde_json::{json, Value};

static EVENT_TABLE: LazyLock<String> =
    LazyLock::new(|| env::var("EVENT_TABLE_ARN").expect("EVENT_TABLE must be set"));

async fn get_event(State(dynamodb): State<aws_sdk_dynamodb::Client>) -> Json<Value> {
    dynamodb.query().table_name(&*EVENT_TABLE);
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

async fn real_main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let api = Router::new()
        .route("/", get(get_event))
        .route("/user_error", get(user_error))
        .route("/server_error", get(server_error))
        .with_state(dynamodb_client);

    let app = Router::new().nest("/api", api);

    run(app).await
}

fn main() -> Result<(), Error> {
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
