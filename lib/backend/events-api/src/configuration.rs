use std::{env, sync::LazyLock};

pub static EVENT_TABLE: LazyLock<String> =
    LazyLock::new(|| env::var("EVENT_TABLE_ARN").expect("EVENT_TABLE must be set"));

pub static EVENT_IMAGES_BUCKET_NAME: LazyLock<String> = LazyLock::new(|| {
    env::var("EVENT_IMAGES_BUCKET_NAME").expect("EVENT_IMAGES_BUCKET_NAME must be set")
});

pub static EVENT_IMAGES_BUCKET_PREFIX: LazyLock<String> = LazyLock::new(|| {
    env::var("EVENT_IMAGES_BUCKET_PREFIX").expect("EVENT_IMAGES_BUCKET_PREFIX must be set")
});

pub static CONTENT_CREATORS_GROUP_NAME: LazyLock<String> = LazyLock::new(|| {
    env::var("CONTENT_CREATORS_GROUP_NAME").expect("CONTENT_CREATORS_GROUP_NAME must be set")
});
