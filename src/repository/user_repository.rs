use std::sync::Arc;

use tokio_stream::StreamExt;

use crate::{
    db::mongo_db::MongoDbConnectionManager,
    models::{
        user_document::UserDocument, user_response::UserResponse,
    },
    utils::{constants::NAME_TABLE_USER, custom_error::CustomError},
};

pub struct UserRepository {
    connection_manager: Arc<MongoDbConnectionManager>,
}

impl UserRepository {
    pub fn new(connection_manager: Arc<MongoDbConnectionManager>) -> Self {
        Self { connection_manager }
    }

    pub async fn get_all(&self) -> Result<Vec<UserResponse>, CustomError> {
        let mut user_documents = self
            .connection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<UserDocument>(NAME_TABLE_USER)
            .find(None, None)
            .await?;
        
        let mut user_responses: Vec<UserResponse> = Vec::new();
        while let Some(user_document) = user_documents.try_next().await? {
            user_responses.push(UserResponse::from(user_document));
            
        } 
        
        Ok(user_responses)
    }
    // pub async fn save(&self, foco_request: FocoRequest) -> Result<FocoRequest, CustomError> {
    //     let foco_document = self.find_by_num(foco_request.number).await?;
        
    //     // si existe actualizamos
    //     if let Some(_) = foco_document {
    //         self.connection_manager
    //             .get_connection()
    //             .await
    //             .get_database()
    //             .collection::<FocoDocument>(NAME_DB_FOCO)
    //             .update_one(
    //                 doc! {"number": foco_request.number},
    //                 doc! {"$set": {"state": foco_request.state.clone()}},
    //                 None,
    //             )
    //             .await?;
    //         return Ok(foco_request.clone());
    //     } else {
    //         // si no existe creamos
    //         let foco_document = FocoDocument::new(foco_request.number, foco_request.state.clone());

    //         self.connection_manager
    //             .get_connection()
    //             .await
    //             .get_database()
    //             .collection::<FocoDocument>(NAME_DB_FOCO)
    //             .insert_one(foco_document, None)
    //             .await?;
    //         Ok(foco_request.clone())
    //     } 
        
    // }

    // pub async fn find_by_num(&self, number: i32) -> Result<Option<FocoDocument>, CustomError> {
    //     let foco_document = self
    //         .connection_manager
    //         .get_connection()
    //         .await
    //         .get_database()
    //         .collection::<FocoDocument>(NAME_DB_FOCO)
    //         .find_one(doc!{"number": number}, None)
    //         .await?;
    //     Ok(foco_document)
    // }
}