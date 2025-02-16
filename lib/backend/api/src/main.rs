use std::{env, sync::LazyLock};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use lambda_http::{run, Error};
use model::{
    database::Event,
    rest::{DatabaseQueryFailed, EventNotFoundError, RestError, UnknownSdkError},
};
use serde_json::{json, Value};
use tracing::{error, info};
use tracing_subscriber::fmt::format;
use uuid::Uuid;

mod model;

static EVENT_TABLE: LazyLock<String> =
    LazyLock::new(|| env::var("EVENT_TABLE_ARN").expect("EVENT_TABLE must be set"));

fn setup_logging() {
    tracing_subscriber::fmt()
        .event_format(format::json().with_thread_ids(false).with_ansi(false))
        .init();
}

async fn get_event(
    Path(event_id): Path<Uuid>,
    State(dynamodb): State<aws_sdk_dynamodb::Client>,
) -> Result<Json<Event>, RestError> {
    info!("Getting event with id {}", event_id);
    let res = dynamodb
        .query()
        .table_name(&*EVENT_TABLE)
        .key_condition_expression("PK = :eventId")
        .expression_attribute_values(
            ":eventId",
            aws_sdk_dynamodb::types::AttributeValue::S(format!("Event#{}", event_id.to_string())),
        )
        .send()
        .await
        .map_err(|s| {
            error!("Failed to query database: {s:?}");
            sentry::capture_error(&s);
            DatabaseQueryFailed.into()
        })?;

    let events = res.items.ok_or_else(|| {
        error!("Got a response, but 'items' field is missing");
        sentry::capture_message(
            "No items in response when listing events",
            sentry::Level::Error,
        );
        UnknownSdkError.into()
    })?;

    let event = events
        .first()
        .ok_or_else(|| EventNotFoundError(event_id.to_string()).into())?;
    info!("Found event {:?}", event_id);

    let event = Event::try_from(event)
        .inspect_err(|e| {
            error!("Failed to parse event: {e:?}");
            sentry::capture_error(e);
        })
        .map_err(|e| e.into())?;
    info!("Parsed event {:?}. Returning it to caller.", event_id);
    Ok(Json(event))
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
