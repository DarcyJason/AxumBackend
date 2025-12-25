use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::domain::{folder::Folder, organization::Organization};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CipherType {
    Login,
    Card,
    Identity,
    Note,
    SSH,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FieldType {
    Text,
    Hide,
    CheckBox,
    Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalField {
    pub field_type: FieldType,
    pub field_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cipher {
    pub id: Thing,
    pub user_id: Thing,
    pub name: String,
    pub cipher_type: CipherType,
    pub folder: Option<Folder>,
    pub organization: Option<Organization>,
    pub data: String,
    pub remarks: Option<String>,
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
