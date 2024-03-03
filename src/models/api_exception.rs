use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct ApiException {
    pub code: StatusCode,
    pub message: String,
    pub component: Option<String>,
}

#[derive(Serialize)]
pub struct ApiExceptionBuilder {
    code: String,
    message: String,
    component: String,
}

impl ApiException {
    pub fn new(code: StatusCode, message: String, component: Option<String>) -> Self {
        Self {
            code,
            message,
            component,
        }
    }
}

impl IntoResponse for ApiException {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ApiExceptionBuilder {
                code: self.code.to_string(),
                message: self.message,
                component: self.component.unwrap_or_default()
            }),
        ).into_response()
    }
}