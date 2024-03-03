use serde::{Serialize, Deserialize};

use super::user_document::UserDocument;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserResponse {
    pub player_id: String,
    pub username: String,
    pub password: String,
    pub currency: String,
    pub platform_id: String,
    pub country: String,
    pub phone: String,
    pub domain: String,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub serial_slot: String,
    pub role: String,
    pub max_bet: String,
    pub min_bet: String,
    pub server: String,
    pub info: String,
    pub parent_id: String,
    pub doc_type: String,
    pub document: String,
    pub birthday: String,
    pub u_serial: String,
    pub city: String,
    pub address: String,
    pub blocked: bool,
    pub created_by: String,
    pub updated_by: String
}

impl UserResponse {
    pub fn from(user_document: UserDocument) -> Self {
        Self {
            player_id: user_document.player_id,
            username: user_document.username,
            password: user_document.password,
            currency: user_document.currency,
            platform_id: user_document.platform_id,
            country: user_document.country,
            phone: user_document.phone,
            domain: user_document.domain,
            name: user_document.name,
            last_name: user_document.last_name,
            email: user_document.email,
            serial_slot: user_document.serial_slot,
            role: user_document.role,
            max_bet: user_document.max_bet,
            min_bet: user_document.min_bet,
            server: user_document.server,
            info: user_document.info,
            parent_id: user_document.parent_id,
            doc_type: user_document.doc_type,
            document: user_document.document,
            birthday: user_document.birthday,
            u_serial: user_document.u_serial,
            city: user_document.city,
            address: user_document.address,
            blocked: user_document.blocked,
            created_by: user_document.created_by,
            updated_by: user_document.updated_by
        }
    }
}