use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TransactionRequest {
    pub trxid: Option<String>,
    pub movement: Option<String>,
    pub amount: f64,
    pub gameid: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    pub brand: String,
    pub category: String,
    #[serde(rename = "referenceBet")]
    pub reference_bet:  Option<String>,
    pub serial: String,
    pub usertoken: String,
}

