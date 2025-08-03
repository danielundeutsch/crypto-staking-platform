use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use std::time::Instant;
use tracing::info;

pub async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, axum::Error> {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;
    
    let duration = start.elapsed();
    info!(
        "{} {} - {} - {:?}",
        method,
        uri,
        response.status(),
        duration
    );

    Ok(response)
}