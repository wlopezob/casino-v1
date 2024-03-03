use axum::{extract::State, routing::{delete, get, post, put}, Json, Router};

use crate::{models::{api_exception::ApiException, user_response::UserResponse}, routes::init::{AppState, UserRepositoryState}, services::foco_service::UserService};

struct UserController {
    user_service: UserService,
}

impl UserController {
    pub fn new(user_repository: UserRepositoryState) -> Self {
        Self { user_service: UserService
            ::new(user_repository)
        }
    }

    pub async fn get_all(State(app_state): State<AppState>) -> 
        Result<Json<Vec<UserResponse>>, ApiException> {
        let controler = UserController::new(app_state
            .user_repository);
        let user_responses = controler.user_service.get_all().await?;
        Ok(Json(user_responses))
    }
}

pub fn user_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(UserController::get_all))
        .route("/", post(register))
        .route("/", put(update))
        .route("/", delete(delete_user))
}




pub async fn register() -> String {
    "register".to_owned()
}

pub async fn update() -> String {
    "update".to_owned()
}


pub async fn delete_user() -> String {
    "delete".to_owned()
}