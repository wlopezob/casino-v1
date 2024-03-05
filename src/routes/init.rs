use std::{env, sync::Arc};

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::{controllers::transaction_controller::user_controller, db, repository::transaction_repository::TransactionRepository, utils::custom_error::CustomError};

pub type Result<T> = core::result::Result<T, CustomError>;

pub type TransactionRepositoryState = Arc<TransactionRepository>;

#[derive(Clone)]
pub struct AppState {
    pub transaction_repository: TransactionRepositoryState,
}

impl AppState {
    pub fn new(transaction_repository: TransactionRepositoryState) -> Self {
        Self { transaction_repository }
    }
}

pub async fn run() {
    let conection_string = env::var("MONGO_DB").expect("Error load conection string");
    let name_database = env::var("NAME_DATABASE").expect("Error load conection string");
    let conection_manager = Arc::new(
        db::mongo_db::MongoDbConnectionManager::new(conection_string, name_database)
            .await
            .expect("Error load database"),
    );
    let foco_repository = Arc::new(TransactionRepository
        ::new(conection_manager.clone()));
    let app_state = AppState::new(foco_repository);

    let cors_layer =  CorsLayer::new().allow_origin(Any);
    let app = init_router(cors_layer, app_state);
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:8080")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_router(cors_layer: CorsLayer, app_state: AppState) -> Router {
    Router::new()
        .nest("/casino-api-v1", chidren_router())
        .with_state(app_state)
        .layer(cors_layer)
}

fn chidren_router() -> Router<AppState> {
    Router::new()
        .nest("/user", user_controller())
}