use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, registry::Registry, util::SubscriberInitExt,
};

pub fn init_tracing() {
    // Get log level from environment variable or default to "info"
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Create a formatting layer
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // Create a JSON formatting layer for structured logging
    let json_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true);

    // Use the environment variable RUST_LOG_JSON=true to enable JSON logging
    let use_json = std::env::var("RUST_LOG_JSON")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    // Initialize the subscriber
    if use_json {
        Registry::default()
            .with(env_filter)
            .with(json_layer)
            .init();
    } else {
        Registry::default()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    }

    tracing::info!("Tracing initialized");
}
