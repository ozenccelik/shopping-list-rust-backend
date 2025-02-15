// Define a middleware to log requests and responses
async fn log_request<B>(req: RequestParts<B>, next: Next<B>) -> impl axum::response::IntoResponse {
    tracing::info!("Request: {} {}", req.method(), req.uri());
    let response = next.run(req).await;
    tracing::info!("Response: {}", response.status());
    response
}
