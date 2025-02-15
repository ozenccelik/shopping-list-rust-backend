use axum::{extract::RequestParts, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth_middleware<B>(
    req: RequestParts<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

    if let Some(token) = token {
        if decode::<serde_json::Value>(
            token,
            &DecodingKey::from_secret("your_secret_key".as_ref()),
            &Validation::default(),
        )
        .is_ok()
        {
            return Ok(next.run(req).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
