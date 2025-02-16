use axum::extract::{Path, State};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    configuration::EVENT_TABLE,
    model::{
        database::Event,
        rest::{DatabaseQueryFailed, EventNotFoundError, RestError, UnknownSdkError},
    },
};

pub async fn get_event(
    Path(event_id): Path<Uuid>,
    State(dynamodb): State<aws_sdk_dynamodb::Client>,
) -> Result<Event, RestError> {
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
    Ok(event)
}
