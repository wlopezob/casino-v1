use crate::{
    models::{
        api_exception::ApiException, transaction_document::TransactionDocument, 
        transaction_request::TransactionRequest, transaction_response::TransactionResponse,
    }, routes::init::TransactionRepositoryState, utils::api_exception_enum::{error_01, error_02, error_03, error_04}
};

pub struct TransactionService {
    transaction_repository: TransactionRepositoryState,
}

impl TransactionService {
    pub fn new(transaction_repository: TransactionRepositoryState) -> Self {
        Self { transaction_repository }
    }

    
    pub async fn get_all(&self) -> Result<Vec<TransactionResponse>, ApiException> {
        let transaction_responses = self
            .transaction_repository
            .get_all()
            .await
            .map_err(|error| error_01(error.to_string()))?;
        Ok(transaction_responses)
    }

    pub async fn save(&self, transaction_request: 
        TransactionRequest) -> Result<bool, ApiException> {
        transaction_request.trxid.clone().ok_or(error_01("El trxId is required".to_string()))?;
        let exist = self
            .transaction_repository
            .exist_trxid(transaction_request.trxid.clone().unwrap())
            .await
            .map_err(|error| error_01(error.to_string()))?;
        if exist {
            return Err(error_01("El trxId already exists".to_string()));
        }

        self.transaction_repository
            .save(TransactionDocument
                ::new(transaction_request.clone()))
            .await
            .map_err(|error| error_01(error.to_string()))?;
        Ok(true)
    }

    pub async fn delete(&self, trx_id: String) -> Result<(), ApiException> {
        self.transaction_repository
            .delete(trx_id)
            .await
            .map_err(|error| error_02(error.to_string()))?;
        Ok(())
    }

    pub async fn get_by_trxid(&self, trx_id: String) -> Result<TransactionResponse, ApiException> {
        let transaction_document = self
            .transaction_repository
            .get_by_trxid(trx_id)
            .await
            .map_err(|error| error_03(error.to_string()))?;
        Ok(TransactionResponse::new(transaction_document))
    }
    
    pub async fn update(&self, trx_id: String, 
            transaction_request: TransactionRequest) -> Result<(), ApiException> {
        self.transaction_repository
            .update(trx_id, TransactionDocument
                ::new(transaction_request.clone()))
            .await
            .map_err(|error| error_04(error.to_string()))?;
        Ok(())
    }
}