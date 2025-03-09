use crate::events::queries::DynamodbQueries;
use axum::extract::{Path, State};
use uuid::Uuid;

use std::collections::HashMap;

use axum::{response::IntoResponse, Json};
use serde::Serialize;

use super::error::RestError;

#[derive(serde::Deserialize, Serialize)]
pub struct Contact {
    organizer: Option<String>,
    email: Option<String>,
    email_visible: bool,
    phone: Option<String>,
}

#[derive(serde::Deserialize, Serialize)]
pub struct Location {
    name: String,
    link: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Uuid,
    pub title: HashMap<String, String>,
    #[serde(with = "time::serde::rfc3339")]
    pub signup_end_date: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub event_date: time::OffsetDateTime,
    pub location: Location,
    pub contact: Contact,
    pub description: HashMap<String, String>,
    pub limit: Option<u16>,
    pub image: Option<Uuid>,
    pub visible: bool,
}

impl IntoResponse for Event {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<crate::events::models::Event> for Event {
    fn from(value: crate::events::models::Event) -> Self {
        let location = Location {
            name: value.location_name,
            link: value.location_link,
        };
        Self {
            id: value.id,
            title: value.title,
            signup_end_date: value.signup_end_date,
            event_date: value.event_date,
            location,
            contact: Contact {
                organizer: value.organizer_name,
                email: if value.email_visible {
                    Some(value.email)
                } else {
                    None
                },
                email_visible: value.email_visible,
                phone: value.phone,
            },
            description: value.description,
            limit: value.limit,
            image: value.image,
            visible: value.event_visible,
        }
    }
}
pub async fn get_event(
    Path(event_id): Path<Uuid>,
    State(dynamodb): State<DynamodbQueries>,
) -> Result<Event, RestError> {
    tracing::debug!("Getting event with id: {}", event_id);
    let event = dynamodb
        .get_event(event_id)
        .await
        .map_err(|e| RestError::from(e))
        .map(|event| {
            tracing::debug!("Found event: {}", event.id);
            return event.into();
        });

    return event;
}
