use api::get_event::get_event;
use authentication::content_creator_authorizer_middleware;
use axum::{
    extract::{DefaultBodyLimit, FromRef},
    middleware,
    routing::{get, put},
    Router,
};
use configuration::EVENT_TABLE;
use events::queries::DynamodbQueries;
use lambda_http::{run, Error};
use tracing_subscriber::{fmt::format, EnvFilter};

mod api;
mod authentication;
mod configuration;
mod database;
mod events;
mod images;

#[derive(Clone)]
struct ApiState {
    dynamodb_queries: events::queries::DynamodbQueries,
    s3_client: aws_sdk_s3::Client,
}

impl FromRef<ApiState> for DynamodbQueries {
    fn from_ref(state: &ApiState) -> DynamodbQueries {
        state.dynamodb_queries.clone()
    }
}

impl FromRef<ApiState> for aws_sdk_s3::Client {
    fn from_ref(state: &ApiState) -> aws_sdk_s3::Client {
        state.s3_client.clone()
    }
}

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(format::json().with_thread_ids(false).with_ansi(false))
        .init();
}

async fn real_main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let dynamodb_queries =
        events::queries::DynamodbQueries::new(dynamodb_client.clone(), &EVENT_TABLE);
    let s3_client = aws_sdk_s3::Client::new(&config);

    let state = ApiState {
        dynamodb_queries,
        s3_client,
    };

    let public_router = Router::new().route("/event/{eventId}", get(get_event));

    let admin_api = Router::new()
        .route("/event/{eventId}/image", put(api::put_image::put_image))
        // 10 mb limit for images
        .layer(DefaultBodyLimit::disable())
        .route_layer(middleware::from_fn(content_creator_authorizer_middleware));

    let app = Router::new()
        .nest("/api/public", public_router)
        .nest("/api/admin", admin_api)
        .with_state(state);

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
