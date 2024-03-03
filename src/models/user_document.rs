use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserDocument {
    #[serde(rename = "playerId")]
    pub player_id: String,
    pub username: String,
    pub password: String,
    pub currency: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub country: String,
    pub phone: String,
    pub domain: String,
    pub name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
    #[serde(rename = "serialSlot")]
    pub serial_slot: String,
    pub role: String,
    #[serde(rename = "maxBet")]
    pub max_bet: String,
    #[serde(rename = "minBet")]
    pub min_bet: String,
    pub server: String,
    pub info: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "docType")]
    pub doc_type: String,
    pub document: String,
    pub birthday: String,
    #[serde(rename = "uSerial")]
    pub u_serial: String,
    pub city: String,
    pub address: String,
    pub blocked: bool,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "updatedBy")]
    pub updated_by: String
}