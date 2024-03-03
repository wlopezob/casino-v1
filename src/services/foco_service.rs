use crate::{
    models::{
        api_exception::ApiException,
        // foco_request::FocoRequest,
        user_response::UserResponse,
    }, routes::init::UserRepositoryState, utils::api_exception_enum::error_01
};

pub struct UserService {
    user_repository: UserRepositoryState,
}

impl UserService {
    pub fn new(user_repository: UserRepositoryState) -> Self {
        Self { user_repository }
    }

    
    pub async fn get_all(&self) -> Result<Vec<UserResponse>, ApiException> {
        let user_responses = self
            .user_repository
            .get_all()
            .await
            .map_err(|error| error_01(error.to_string()))?;
        Ok(user_responses)
    }
}