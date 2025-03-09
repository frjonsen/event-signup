use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, ScalarAttributeType};
use serde::Deserialize;
use uuid::Uuid;

use crate::database::{
    errors::ModelError,
    util::{
        get_boolean, get_datetime, get_delimited, get_field, get_nested_object, get_optional_field,
    },
};

pub mod columns {
    include!(concat!(env!("OUT_DIR"), "/db_structure.rs"));
}

pub mod column_aliases {
    use super::columns;

    pub const ID: &str = columns::PARTITION_KEY_COLUMN;
}

pub struct Event {
    pub id: Uuid,
    pub title: HashMap<String, String>,
    pub signup_end_date: time::OffsetDateTime,
    pub event_date: time::OffsetDateTime,
    pub creator_username: String,
    pub description: HashMap<String, String>,
    pub limit: Option<u16>,
    pub image: Option<Uuid>,
    pub event_visible: bool,
    pub phone: Option<String>,
    pub email: String,
    pub email_visible: bool,
    pub organizer_name: Option<String>,
    pub location_name: String,
    pub location_link: String,
}

impl Event {
    pub const SORT_KEY_VALUE: &str = "Event";
}

impl TryFrom<&HashMap<String, AttributeValue>> for Event {
    type Error = ModelError;
    fn try_from(item: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id: Uuid = get_delimited(item, column_aliases::ID)?;

        Ok(Self {
            id,
            signup_end_date: get_datetime(item, columns::SIGNUP_DEADLINE_COLUMN)?,
            event_date: get_datetime(item, columns::EVENT_DATE_COLUMN)?,
            creator_username: get_field(item, columns::CREATOR_COLUMN)?,
            description: get_nested_object(item, columns::DESCRIPTION_COLUMN)?,
            title: get_nested_object(item, columns::TITLE_COLUMN)?,
            limit: get_optional_field(
                item,
                columns::PARTICIPANTS_LIMIT_COLUMN,
                ScalarAttributeType::N,
            )?,
            email: get_field(item, columns::EMAIL_COLUMN)?,
            email_visible: get_boolean(item, columns::EMAIL_VISIBLE_COLUMN)?,
            phone: get_optional_field(item, columns::PHONE_COLUMN, ScalarAttributeType::S)?,
            location_name: get_field(item, columns::LOCATION_NAME_COLUMN)?,
            location_link: get_field(item, columns::LOCATION_LINK_COLUMN)?,
            image: get_optional_field(item, columns::IMAGE_COLUMN, ScalarAttributeType::S)?,
            event_visible: get_boolean(item, columns::EVENT_VISIBLE_COLUMN)?,
            organizer_name: get_optional_field(item, columns::NAME_COLUMN, ScalarAttributeType::S)?,
        })
    }
}
