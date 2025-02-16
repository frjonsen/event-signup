use std::{env, sync::LazyLock};

pub static EVENT_TABLE: LazyLock<String> =
    LazyLock::new(|| env::var("EVENT_TABLE_ARN").expect("EVENT_TABLE must be set"));
