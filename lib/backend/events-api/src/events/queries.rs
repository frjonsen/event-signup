use std::collections::HashMap;

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

#[derive(Clone)]
pub struct DynamodbQueries {
    client: aws_sdk_dynamodb::Client,
    table_name: &'static str,
}

impl DynamodbQueries {
    pub fn new(client: aws_sdk_dynamodb::Client, table_name: &'static str) -> Self {
        Self { client, table_name }
    }

    pub async fn set_event_image(
        &self,
        event_id: uuid::Uuid,
        image_id: uuid::Uuid,
    ) -> Result<(), AddImageError> {
        self.client
            .update_item()
            .table_name(self.table_name)
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

    pub async fn get_event(&self, event_id: uuid::Uuid) -> Result<Event, GetEventError> {
        let res = self
            .client
            .query()
            .table_name(self.table_name)
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aws_config::Region;
    use aws_sdk_dynamodb::types::{
        builders::KeySchemaElementBuilder, AttributeDefinition, AttributeValue, BillingMode,
    };
    use serde_json::Value;
    use testcontainers_modules::{
        localstack::LocalStack,
        testcontainers::{runners::AsyncRunner, ContainerAsync, ImageExt},
    };
    use uuid::Uuid;

    use crate::events::models::columns;

    fn json_to_dynamodb(json: serde_json::Value) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        if let serde_json::Value::Object(map) = json {
            for (key, value) in map {
                let attr_value = match value {
                    serde_json::Value::String(s) => AttributeValue::S(s),
                    serde_json::Value::Number(n) => AttributeValue::N(n.to_string()),
                    serde_json::Value::Bool(b) => AttributeValue::Bool(b),
                    _ => continue, // Skip unsupported types
                };
                item.insert(key, attr_value);
            }
        }
        item
    }

    async fn init_dynamodb() -> (ContainerAsync<LocalStack>, aws_sdk_dynamodb::Client) {
        let request = LocalStack::default().with_env_var("SERVICES", "dynamodb");
        let container = request.start().await.expect("Failed to start localstack");

        let endpoint_url = format!(
            "http://{}:{}",
            container
                .get_host()
                .await
                .expect("Failed to get local stack host"),
            container
                .get_host_port_ipv4(4566)
                .await
                .expect("Failed to get local stack port")
        );
        let creds = aws_sdk_dynamodb::config::Credentials::new("fake", "fake", None, None, "test");
        let config = aws_sdk_dynamodb::config::Builder::default()
            .behavior_version_latest()
            .credentials_provider(creds)
            .region(Region::new("us-east-1"))
            .endpoint_url(endpoint_url)
            .build();

        let client = aws_sdk_dynamodb::Client::from_conf(config);

        let pk_attribute = AttributeDefinition::builder()
            .attribute_name(columns::PARTITION_KEY_COLUMN)
            .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
            .build()
            .expect("Failed to build test table attribute definition");
        let sk_attribute = AttributeDefinition::builder()
            .attribute_name(columns::SORTING_KEY_COLUMN)
            .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
            .build()
            .expect("Failed to build test table attribute definition");

        let pk_schema = KeySchemaElementBuilder::default()
            .attribute_name("PK")
            .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
            .build()
            .expect("Failed to build test table key schema");
        let sk_schema = KeySchemaElementBuilder::default()
            .attribute_name("SK")
            .key_type(aws_sdk_dynamodb::types::KeyType::Range)
            .build()
            .expect("Failed to build test table key schema");

        client
            .create_table()
            .table_name("events")
            .attribute_definitions(pk_attribute)
            .attribute_definitions(sk_attribute)
            .billing_mode(BillingMode::PayPerRequest)
            .key_schema(pk_schema)
            .key_schema(sk_schema)
            .send()
            .await
            .expect("Failed to create test table");

        (container, client)
    }

    async fn insert_test_event(client: &aws_sdk_dynamodb::Client) -> Uuid {
        let event: Value =
            serde_json::from_str(include_str!("../test_fixtures/event.json")).unwrap();
        let event_id = Uuid::parse_str(
            event
                .as_object()
                .unwrap()
                .get("PK")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap();
        let item = json_to_dynamodb(event);
        client
            .put_item()
            .table_name("events")
            .set_item(Some(item))
            .send()
            .await
            .expect("Failed to insert test event");

        event_id
    }

    #[tokio::test]
    async fn test_get_event() {
        let (_container, client) = init_dynamodb().await;
        let event_id = insert_test_event(&client).await;

        let queries = super::DynamodbQueries::new(client, "events");
        let event_from_db = queries
            .get_event(event_id)
            .await
            .expect("Failed to get event from database");

        assert_eq!(event_from_db.id, event_id);
    }

    #[tokio::test]
    async fn test_set_event_image() {
        let (_container, client) = init_dynamodb().await;
        let new_image_id = Uuid::new_v4();
        let event_id = insert_test_event(&client).await;
        let queries = super::DynamodbQueries::new(client, "events");
        let event_from_db = queries
            .get_event(event_id)
            .await
            .expect("Failed to get event from database");
        assert_ne!(event_from_db.image.unwrap(), new_image_id);
        queries
            .set_event_image(event_id, new_image_id)
            .await
            .expect("Failed to add image to event");
        let updated_event = queries
            .get_event(event_id)
            .await
            .expect("Failed to get event from database");
        assert_eq!(updated_event.image.unwrap(), new_image_id);
    }
}
