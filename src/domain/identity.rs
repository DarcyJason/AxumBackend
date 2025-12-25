use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Status {
    Husband,
    Wife,
    Sir,
    Madam,
    Doctor,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Thing,
    pub status: Status,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub company: Option<String>,
    pub social_security_number: Option<String>,
    pub passport_number: Option<String>,
    pub license_number: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub post_code: Option<String>,
    pub country: Option<String>,
}
