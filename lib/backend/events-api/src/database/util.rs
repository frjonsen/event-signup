use std::{collections::HashMap, str::FromStr};

use aws_sdk_dynamodb::types::{AttributeValue, ScalarAttributeType};
use serde::de::DeserializeOwned;
use time::OffsetDateTime;

use super::errors::ModelError;

fn get_string_internal<'a>(
    item: &'a HashMap<String, AttributeValue>,
    field: &str,
) -> Result<&'a String, ModelError> {
    item.get(field)
        .ok_or_else(|| ModelError::MissingField(field.to_owned()))?
        .as_s()
        .map_err(|_| ModelError::InvalidData(format!("{field} field is not a string")))
}

pub fn get_boolean(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<bool, ModelError> {
    item.get(field)
        .ok_or_else(|| ModelError::MissingField(field.to_owned()))?
        .as_bool()
        .map_err(|_| ModelError::InvalidData(format!("{field} is not a bool")))
        .map(bool::clone)
}

pub fn get_field<T>(item: &HashMap<String, AttributeValue>, field: &str) -> Result<T, ModelError>
where
    T: FromStr,
{
    let read_value = get_string_internal(item, field)?;
    let parsed_value = read_value
        .parse()
        .map_err(|_| ModelError::InvalidGenericType(field.to_owned(), read_value.to_owned()))?;
    Ok(parsed_value)
}

pub fn get_list<T>(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<Vec<T>, ModelError>
where
    T: FromStr,
{
    let read_value = match item.get(field) {
        None => return Ok(Vec::new()),
        Some(f) => f,
    }
    .as_ss()
    .map_err(|_| ModelError::InvalidData(format!("{field} is not a string list")))?;

    let mut parsed_values = Vec::new();
    for value in read_value.into_iter() {
        parsed_values.push(
            value
                .parse()
                .map_err(|_| ModelError::InvalidGenericType(field.to_owned(), value.to_owned()))?,
        );
    }

    Ok(parsed_values)
}

pub fn get_optional_field<T>(
    item: &HashMap<String, AttributeValue>,
    field: &str,
    field_type: ScalarAttributeType,
) -> Result<Option<T>, ModelError>
where
    T: FromStr,
{
    let value = match item.get(field) {
        Some(f) => f,
        None => return Ok(None),
    };
    let value: &String = match field_type {
        ScalarAttributeType::N => value.as_n(),
        ScalarAttributeType::S => value.as_s(),
        _ => {
            return Err(ModelError::InvalidData(
                "Other types than numbers or strings are not supported".to_owned(),
            ))
        }
    }
    .map_err(|_| ModelError::InvalidData(format!("{field} field is not a {field_type}")))?;
    let value = value
        .parse()
        .map_err(|_| ModelError::InvalidGenericType(field.to_owned(), value.to_owned()))?;
    Ok(Some(value))
}

pub fn get_nested_object<T>(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<T, ModelError>
where
    T: DeserializeOwned,
{
    let value = get_string_internal(item, field)?;
    serde_json::from_str(value).map_err(|e| {
        sentry::capture_error(&e);
        ModelError::InvalidGenericType(field.to_owned(), value.to_owned())
    })
}

pub fn get_delimited<T>(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<T, ModelError>
where
    T: FromStr,
{
    let read_field: &str = get_string_internal(item, field)?
        .split("#")
        .last()
        .ok_or_else(|| ModelError::MissingDelimiter(field.to_owned()))?;
    let parsed_field = read_field
        .parse()
        .map_err(|_| ModelError::InvalidGenericType(field.to_owned(), read_field.to_owned()))?;

    Ok(parsed_field)
}

pub fn get_datetime(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<OffsetDateTime, ModelError> {
    let read_field = get_string_internal(item, field)?;
    OffsetDateTime::parse(read_field, &time::format_description::well_known::Rfc3339).map_err(
        |_| {
            ModelError::InvalidType(
                field.to_owned(),
                "Rfc3339".to_owned(),
                read_field.to_owned(),
            )
        },
    )
}

pub fn get_optional_datetime(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<Option<OffsetDateTime>, ModelError> {
    if !item.contains_key(field) {
        return Ok(None);
    }
    get_datetime(item, field).map(Some)
}

pub fn get_delimited_datetime(
    item: &HashMap<String, AttributeValue>,
    field: &str,
) -> Result<OffsetDateTime, ModelError> {
    let read_field = get_string_internal(item, field)?
        .split("#")
        .last()
        .ok_or_else(|| ModelError::MissingDelimiter(field.to_owned()))?;
    OffsetDateTime::parse(read_field, &time::format_description::well_known::Rfc3339).map_err(
        |_| {
            ModelError::InvalidType(
                field.to_owned(),
                "Rfc3339".to_owned(),
                read_field.to_owned(),
            )
        },
    )
}
