use api::get_event::get_event;
use axum::{
    extract::{DefaultBodyLimit, FromRef},
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use tracing_subscriber::fmt::format;

mod api;
mod configuration;
mod events;
mod images;
mod model;

#[derive(Clone)]
struct ApiState {
    dynamodb_client: aws_sdk_dynamodb::Client,
    s3_client: aws_sdk_s3::Client,
}

impl FromRef<ApiState> for aws_sdk_dynamodb::Client {
    fn from_ref(state: &ApiState) -> aws_sdk_dynamodb::Client {
        state.dynamodb_client.clone()
    }
}

impl FromRef<ApiState> for aws_sdk_s3::Client {
    fn from_ref(state: &ApiState) -> aws_sdk_s3::Client {
        state.s3_client.clone()
    }
}

fn setup_logging() {
    tracing_subscriber::fmt()
        .event_format(format::json().with_thread_ids(false).with_ansi(false))
        .init();
}

async fn real_main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let s3_client = aws_sdk_s3::Client::new(&config);

    let state = ApiState {
        dynamodb_client,
        s3_client,
    };

    let public_router = Router::new().route("/event/{eventId}", get(get_event));

    let admin_api = Router::new()
        .route("/event/{eventId}/images", post(api::post_image::post_image))
        // 10 mb limit for images
        .layer(DefaultBodyLimit::disable());

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
