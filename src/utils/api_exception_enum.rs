use axum::http::StatusCode;

use crate::models::api_exception::ApiException;

const COMPONENT: &str = "casino-api-v1";

pub fn error_01(msg: String) -> ApiException {
    ApiException::new(StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Error registrando la transaction, error {}", msg), 
        Some(COMPONENT.to_string()))
}

pub fn error_02(msg: String) -> ApiException {
    ApiException::new(StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Error eliminado la transaction, error {}", msg), 
        Some(COMPONENT.to_string()))
}

pub fn error_03(msg: String) -> ApiException {
    ApiException::new(StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Error obteniendo la transaction, error {}", msg), 
        Some(COMPONENT.to_string()))
}

pub fn error_04(msg: String) -> ApiException {
    ApiException::new(StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Error actualizando la transaction, error {}", msg), 
        Some(COMPONENT.to_string()))
}