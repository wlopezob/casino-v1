use std::sync::Arc;

use mongodb::bson::doc;
use tokio_stream::StreamExt;

use crate::{
    db::mongo_db::MongoDbConnectionManager,
    models::{
        transaction_document::TransactionDocument, transaction_response::TransactionResponse},
    utils::{constants::NAME_TABLE_TRANSACTION, custom_error::CustomError},
};

pub struct TransactionRepository {
    connection_manager: Arc<MongoDbConnectionManager>,
}

impl TransactionRepository {
    pub fn new(connection_manager: Arc<MongoDbConnectionManager>) -> Self {
        Self { connection_manager }
    }

    pub async fn get_all(&self) -> Result<Vec<TransactionResponse>, CustomError> {
        let mut transaction_documents = self
            .connection_manager
            .get_connection()
            .get_database()
            .collection::<TransactionDocument>(NAME_TABLE_TRANSACTION)
            .find(None, None)
            .await?;
        
        let mut transactions_responses: Vec<TransactionResponse> = Vec::new();
        while let Some(transaction_document) = transaction_documents.try_next().await? {
            transactions_responses.push(TransactionResponse::new(transaction_document));
            
        } 
        
        Ok(transactions_responses)
    }

    pub async fn get_by_trxid(&self, trx_id: String) -> Result<TransactionDocument, CustomError> {
        let transaction_document = self
            .connection_manager
            .get_connection()
            .get_database()
            .collection::<TransactionDocument>(NAME_TABLE_TRANSACTION)
            .find_one(doc!{"trxId": trx_id}, None)
            .await?
            .ok_or(CustomError::Generic("Transaction not found".to_string()))?;
        Ok(transaction_document)
    }

    pub async fn exist_trxid(&self, trx_id: String) -> Result<bool, CustomError> {
        let user_document = self
            .connection_manager
            .get_connection()
            .get_database()
            .collection::<TransactionDocument>(NAME_TABLE_TRANSACTION)
            .find_one(doc!{"trxId": trx_id}, None)
            .await?;
        Ok(user_document.is_some())
    }

    pub async fn save(&self, transaction_document: TransactionDocument) -> Result<TransactionDocument, CustomError> {
        self.connection_manager
            .get_connection()
            .get_database()
            .collection::<TransactionDocument>(NAME_TABLE_TRANSACTION)
            .insert_one(transaction_document.clone(), None)
            .await?;
        Ok(transaction_document)
    }

    pub async fn delete(&self, trx_id: String) -> Result<(), CustomError> {
        self.connection_manager
            .get_connection()
            .get_database()
            .collection::<TransactionDocument>(NAME_TABLE_TRANSACTION)
            .delete_one(doc!{"trxId": trx_id}, None)
            .await?;
        Ok(())
    }

    pub async fn update(&self, trx_id: String, transaction_document: TransactionDocument) -> Result<(), CustomError> {
        self.connection_manager
            .get_connection()
            .get_database()
            .collection::<TransactionDocument>(NAME_TABLE_TRANSACTION)
            .update_one(doc!{"trxId": trx_id}, 
                doc!{"$set": {"movement": transaction_document.movement, 
                    "amount": transaction_document.amount, 
                    "gameId": transaction_document.game_id, 
                    "game_name": transaction_document.game_name, 
                    "brand": transaction_document.brand, 
                    "category": transaction_document.category, 
                    "referenceBet": transaction_document.reference_bet, 
                    "serial": transaction_document.serial, 
                    "usertoken": transaction_document.usertoken}
                }, 
                None)
            .await?;
        Ok(())
    }
}