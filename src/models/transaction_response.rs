use serde::{Serialize, Deserialize};

use super::transaction_document::TransactionDocument;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TransactionResponse {
    #[serde(rename = "trxId")]
    pub trx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement: Option<String>,
    pub amount: f64,
    // #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    pub brand: String,
    pub category: String,
    // #[serde(rename = "referenceBet")]
    pub reference_bet:  Option<String>,
    pub serial: String,
    #[serde(rename = "usertoken")]
    pub usertoken: String,
}

impl TransactionResponse {
    pub fn new(transaction_document: TransactionDocument) -> Self {
        Self {
            trx_id: transaction_document.trx_id,
            movement: transaction_document.movement,
            amount: transaction_document.amount
                .unwrap_or("0".parse().unwrap()).to_string().parse().unwrap(),
            game_id: transaction_document.game_id,
            game_name: transaction_document.game_name,
            brand: transaction_document.brand,
            category: transaction_document.category,
            reference_bet: transaction_document.reference_bet,
            serial: transaction_document.serial,
            usertoken: transaction_document.usertoken,
        }
    }
}
