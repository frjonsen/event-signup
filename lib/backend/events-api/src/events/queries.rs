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

    pub async fn add_image_to_event(
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
    use aws_config::Region;
    use aws_sdk_dynamodb::types::{
        builders::KeySchemaElementBuilder, AttributeDefinition, BillingMode,
    };
    use testcontainers_modules::{
        localstack::LocalStack,
        testcontainers::{runners::AsyncRunner, ImageExt},
    };

    use crate::events::models::columns;

    #[tokio::test]
    async fn test_get_event() {
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

        let mut queries = super::DynamodbQueries::new(client, "events");
    }
}
