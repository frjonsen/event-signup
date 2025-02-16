use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, ScalarAttributeType};
use serde::Serialize;
use uuid::Uuid;

use super::{
    errors::ModelError,
    util::{
        get_datetime, get_delimited, get_delimited_datetime, get_field, get_nested_object,
        get_optional_field,
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
    signup_end_date: time::OffsetDateTime,
    event_date: time::OffsetDateTime,
    admin_id: String,
    contact: Contact,
    description: HashMap<String, String>,
    limit: Option<u16>,
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
        })
    }
}
