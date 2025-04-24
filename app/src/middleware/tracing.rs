use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, instrument, Level};

pub fn create_tracing_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    info!("Creating tracing layer");
    
    TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .level(Level::INFO)
                .include_headers(true),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .include_headers(true),
        )
}

#[instrument(skip(request, next))]
pub async fn request_tracing_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_owned();
    let method = request.method().clone();
    
    info!(
        method = %method,
        path = %path,
        "Request started"
    );
    
    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    info!(
        method = %method,
        path = %path,
        status = %response.status(),
        duration = ?duration,
        "Request completed"
    );
    
    Ok(response)
}
