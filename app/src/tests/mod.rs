#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use infrastructure::config::EnvConfigProvider;
    use infrastructure::persistence::memory::InMemoryUserRepository;
    use infrastructure::security::{BcryptPasswordService, JwtServiceImpl};

    #[tokio::test]
    async fn test_app_state_creation() {
        // Load configuration
        let config_provider = Arc::new(EnvConfigProvider::new().unwrap());

        // Create repositories
        let memory_user_repo = Arc::new(InMemoryUserRepository::new());

        // Create services
        let password_service = Arc::new(BcryptPasswordService::new(None));
        let jwt_service = Arc::new(JwtServiceImpl::new(Arc::clone(&config_provider)));

        // Create application services
        let auth_service = Arc::new(application::services::AuthServiceImpl::new(
            Arc::clone(&memory_user_repo), // Use in-memory repository for testing
            Arc::clone(&jwt_service),
            Arc::clone(&password_service),
        ));

        let user_service = Arc::new(application::services::UserServiceImpl::new(
            Arc::clone(&memory_user_repo), // Use in-memory repository for testing
            Arc::clone(&password_service),
        ));

        // Create use cases
        let auth_use_cases = Arc::new(application::use_cases::AuthUseCases::new(
            Arc::clone(&auth_service),
        ));

        let user_use_cases = Arc::new(application::use_cases::UserUseCases::new(
            Arc::clone(&user_service),
        ));

        // Build the application state
        let app_state = crate::api::AppState {
            auth_use_cases,
            user_use_cases,
            jwt_service: Arc::clone(&jwt_service),
            config_provider: Arc::clone(&config_provider),
        };

        // Build the router
        let _app = crate::api::create_router(app_state);

        // If we get here without panicking, the test passes
        assert!(true);
    }
}
