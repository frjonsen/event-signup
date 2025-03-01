use tracing::{error, info};

use crate::{
    configuration::EVENT_TABLE,
    model::database::errors::{DatabaseQueryFailed, UnknownSdkError},
};

use super::{errors::GetEventError, models::Event};

async fn get_event(
    dynamodb: &aws_sdk_dynamodb::Client,
    event_id: &uuid::Uuid,
) -> Result<Event, GetEventError> {
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
            GetEventError::from(DatabaseQueryFailed)
        })?;

    let events = res.items.ok_or_else(|| {
        error!("Got a response, but 'items' field is missing");
        sentry::capture_message(
            "No items in response when listing events",
            sentry::Level::Error,
        );
        GetEventError::from(UnknownSdkError(
            "Got a response, but 'items' field is missing".to_owned(),
        ))
    })?;

    let event = events.first().ok_or_else(|| GetEventError::NotFound)?;
    info!("Found event {:?}", event_id);

    match Event::try_from(event) {
        Ok(event) => Ok(event),
        Err(e) => {
            error!("Failed to parse event: {e:?}");
            sentry::capture_error(&e);
            Err(GetEventError::InvalidStoredEvent(event_id.clone()))
        }
    }
}
