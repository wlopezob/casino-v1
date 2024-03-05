use mongodb::bson::Decimal128;
use serde::{Serialize, Deserialize};
use super::transaction_request::TransactionRequest;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TransactionDocument {
    #[serde(rename = "trxId")]
    pub trx_id: Option<String>,
    pub movement: Option<String>,
    #[serde(rename = "amount")]
    pub amount: Option<Decimal128>,
    #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    pub brand: String,
    pub category: String,
    #[serde(rename = "referenceBet")]
    pub reference_bet: Option<String>,
    pub serial: String,
    #[serde(rename = "usertoken")]
    pub usertoken: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Amount {
    #[serde(rename = "$numberDecimal")]
    pub number_decimal: String,
}

impl TransactionDocument {
    pub fn new(transacion_request: TransactionRequest) -> Self {
        Self{
            trx_id: transacion_request.trxid,
            movement: transacion_request.movement,
            amount: transacion_request.amount.to_string().parse().ok(),
            game_id: transacion_request.gameid,
            game_name: transacion_request.game_name,
            brand: transacion_request.brand,
            category: transacion_request.category,
            reference_bet: transacion_request.reference_bet,
            serial: transacion_request.serial,
            usertoken: transacion_request.usertoken,
        }
    }
}