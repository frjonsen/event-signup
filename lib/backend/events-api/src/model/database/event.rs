use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, ScalarAttributeType};
use axum::{response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;

use super::{
    errors::ModelError,
    util::{
        get_datetime, get_delimited, get_delimited_datetime, get_field, get_list,
        get_nested_object, get_optional_field,
    },
};

mod fields {
    pub const ID: &str = "PK";
    pub const EVENT_DATE: &str = "SK";
    pub const EVENT_CREATOR: &str = "EventCreator";
    pub const DESCRIPTION: &str = "Description";
    pub const TITLE: &str = "Title";
    pub const PARTICIPANT_LIMIT: &str = "ParticipantLimit";
    pub const CONTACT: &str = "Contact";
    pub const SIGNUP_END_DATE: &str = "SignupEndDate";
    pub const PHOTO_ID: &str = "PhotoId";
}

#[derive(serde::Deserialize, Serialize)]
pub struct Contact {
    email: String,
    phone: String,
}

#[derive(serde::Serialize)]
pub struct Event {
    id: Uuid,
    title: HashMap<String, String>,
    #[serde(with = "time::serde::rfc3339")]
    signup_end_date: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    event_date: time::OffsetDateTime,
    admin_id: String,
    contact: Contact,
    description: HashMap<String, String>,
    limit: Option<u16>,
    photo_id: Vec<Uuid>,
}

impl IntoResponse for Event {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for Event {
    type Error = ModelError;
    fn try_from(item: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id: Uuid = get_delimited(item, fields::ID)?;

        Ok(Self {
            id,
            signup_end_date: get_datetime(item, fields::SIGNUP_END_DATE)?,
            event_date: get_delimited_datetime(item, fields::EVENT_DATE)?,
            admin_id: get_field(item, fields::EVENT_CREATOR)?,
            description: get_nested_object(item, fields::DESCRIPTION)?,
            title: get_nested_object(item, fields::TITLE)?,
            limit: get_optional_field(item, fields::PARTICIPANT_LIMIT, ScalarAttributeType::N)?,
            contact: get_nested_object(item, fields::CONTACT)?,
            photo_id: get_list(item, fields::PHOTO_ID)?,
        })
    }
}
