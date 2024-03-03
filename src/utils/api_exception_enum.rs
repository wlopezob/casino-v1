use axum::http::StatusCode;

use crate::models::api_exception::ApiException;

const COMPONENT: &str = "casino-api-v1";

pub fn error_01(msg: String) -> ApiException {
    ApiException::new(StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Error listando los usuarios, error {}", msg), 
        Some(COMPONENT.to_string()))
}

