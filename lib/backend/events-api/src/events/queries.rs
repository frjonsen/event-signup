use time::format_description;
use tracing::{error, info};

use crate::{
    configuration::EVENT_TABLE,
    model::database::errors::{DatabaseQueryFailed, UnknownSdkError},
};

use super::{
    errors::{AddImagesError, GetEventError},
    models::{fields::PHOTOES, Event},
};

pub async fn add_images_to_event(
    dynamodb: &aws_sdk_dynamodb::Client,
    event_id: &uuid::Uuid,
    image_ids: &[uuid::Uuid],
) -> Result<(), AddImagesError> {
    let event = get_event(dynamodb, event_id).await?;
    let key = event.id_as_pk();
    let sort_key = event.event_date_as_sk();
    let set_additions = aws_sdk_dynamodb::types::AttributeValue::Ss(
        image_ids.iter().map(|id| id.to_string()).collect(),
    );

    dynamodb
        .update_item()
        .table_name(&*EVENT_TABLE)
        .key("PK", aws_sdk_dynamodb::types::AttributeValue::S(key))
        .key("SK", aws_sdk_dynamodb::types::AttributeValue::S(sort_key))
        .update_expression("ADD #P :images")
        .expression_attribute_names("#P", PHOTOES)
        .expression_attribute_values(":images", set_additions)
        .send()
        .await
        .map_err(|s| {
            error!("Failed to query database: {s:?}");
            sentry::capture_error(&s);
            AddImagesError::from(DatabaseQueryFailed)
        })?;

    Ok(())
}

pub async fn get_event(
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

impl Event {
    fn id_as_pk(&self) -> String {
        format!("Event#{}", self.id)
    }

    fn event_date_as_sk(&self) -> String {
        let format_description =
            format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z")
                .expect("Invalid format description");
        format!(
            "EventDate#{}",
            self.event_date.format(&format_description).unwrap()
        )
    }
}
