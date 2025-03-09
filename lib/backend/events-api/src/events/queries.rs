use aws_sdk_dynamodb::types::AttributeValue;
use tracing::error;

use crate::{
    configuration::EVENT_TABLE,
    database::errors::{DatabaseQueryFailed, UnknownSdkError},
};

use super::{
    errors::{AddImageError, GetEventError},
    models::{columns::IMAGE_COLUMN, Event},
};

pub async fn add_image_to_event(
    dynamodb: &aws_sdk_dynamodb::Client,
    event_id: uuid::Uuid,
    image_id: uuid::Uuid,
) -> Result<(), AddImageError> {
    dynamodb
        .update_item()
        .table_name(&*EVENT_TABLE)
        .key("PK", AttributeValue::S(event_id.to_string()))
        .key("SK", AttributeValue::S(Event::SORT_KEY_VALUE.to_owned()))
        .update_expression("SET #P = :image")
        .expression_attribute_names("#P", IMAGE_COLUMN)
        .expression_attribute_values(":image", AttributeValue::S(image_id.to_string()))
        .send()
        .await
        .map_err(|s| {
            error!("Failed to query database: {s:?}");
            sentry::capture_error(&s);
            AddImageError::from(DatabaseQueryFailed)
        })?;

    Ok(())
}

pub async fn get_event(
    dynamodb: &aws_sdk_dynamodb::Client,
    event_id: uuid::Uuid,
) -> Result<Event, GetEventError> {
    let res = dynamodb
        .query()
        .table_name(&*EVENT_TABLE)
        .key_condition_expression("PK = :eventId")
        .expression_attribute_values(":eventId", AttributeValue::S(event_id.to_string()))
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

    let event = events.first().ok_or_else(|| {
        tracing::debug!("Failed to find event with id: {event_id}");
        return GetEventError::NotFound;
    })?;
    tracing::debug!("Found event {:?}", event_id);
    match Event::try_from(event) {
        Ok(event) => Ok(event),
        Err(e) => {
            error!("Failed to parse event: {e:?}");
            sentry::capture_error(&e);
            Err(GetEventError::InvalidStoredEvent(event_id.clone()))
        }
    }
}
