use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    models::{
        api_exception::ApiException, transaction_request::TransactionRequest,
        transaction_response::TransactionResponse,
    },
    routes::init::{AppState, TransactionRepositoryState},
    services::transaction_service::TransactionService,
};

struct TransactionController {
    transaction_service: TransactionService,
}

impl TransactionController {
    pub fn new(transaction_repository: TransactionRepositoryState) -> Self {
        Self {
            transaction_service: TransactionService::new(transaction_repository),
        }
    }

    pub async fn save(
        State(app_state): State<AppState>,
        Json(transaction_request): Json<TransactionRequest>,
    ) -> Result<(), ApiException> {
        let controller = TransactionController::new(app_state.transaction_repository);
        controller
            .transaction_service
            .save(transaction_request)
            .await?;

        Ok(())
    }

    pub async fn get_all(
        State(app_state): State<AppState>,
    ) -> Result<Json<Vec<TransactionResponse>>, ApiException> {
        let controler = TransactionController::new(app_state.transaction_repository);
        let transaction_responses = controler.transaction_service.get_all().await?;
        Ok(Json(transaction_responses))
    }

    pub async fn delete(
        State(app_state): State<AppState>,
        Path(id): Path<String>,
    ) -> Result<(), ApiException> {
        let controller = TransactionController::new(app_state.transaction_repository);
        controller.transaction_service.delete(id).await?;
        Ok(())
    }

    pub async fn get_by_trxid(
        State(app_state): State<AppState>,
        Path(id): Path<String>,
    ) -> Result<Json<TransactionResponse>, ApiException> {
        let controller = TransactionController::new(app_state.transaction_repository);
        let transaction_response = controller.transaction_service.get_by_trxid(id).await?;
        Ok(Json(transaction_response))
    }

    pub async fn update(
        State(app_state): State<AppState>,
        Path(id): Path<String>,
        Json(transaction_request): Json<TransactionRequest>,
    ) -> Result<(), ApiException> {
        let controller = TransactionController::new(app_state.transaction_repository);
        controller
            .transaction_service
            .update(id, transaction_request)
            .await?;
        Ok(())
    }
}

pub fn user_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(TransactionController::get_all))
        .route("/:id", get(TransactionController::get_by_trxid))
        .route("/", post(TransactionController::save))
        .route("/:id", put(TransactionController::update))
        .route("/:id", delete(TransactionController::delete))
}
