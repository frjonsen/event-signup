use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{events::models::Event, model::rest::RestError};

pub async fn get_event(
    Path(event_id): Path<Uuid>,
    State(dynamodb): State<aws_sdk_dynamodb::Client>,
) -> Result<Event, RestError> {
}
