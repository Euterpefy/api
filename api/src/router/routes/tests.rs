use axum::{ response::Response, body::{ Body, to_bytes } };
use serde::de::DeserializeOwned;

use std::result::Result;

#[allow(dead_code)]
pub async fn extract_response_body<T>(response: Response<Body>) -> Result<T, String>
    where T: DeserializeOwned
{
    // Convert the body to bytes
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.map_err(|e| e.to_string())?;

    // Convert bytes to String
    let body_string = String::from_utf8(body_bytes.to_vec()).map_err(|e| e.to_string())?;

    // Deserialize the string directly into T
    serde_json::from_str(&body_string).map_err(|e| e.to_string())
}
