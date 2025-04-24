use infrastructure::config::AppConfig;
use std::sync::Arc;

pub struct AppConfigProvider {
    config: Arc<AppConfig>,
}

impl AppConfigProvider {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }
}
