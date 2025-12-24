use surrealdb::sql::Thing;

use chrono::{DateTime, Utc};

use crate::domain::{
    folder::Folder,
    month::Month,
    project::{AdditionalField, ProjectOwner},
};

pub enum CardBrand {
    Visa,
    Mastercard,
    AmericanExpress,
    Discover,
    DinerClub,
    JCB,
    Maestro,
    UnionPay,
    RuPay,
    Others,
}

pub struct Card {
    pub id: Thing,
    pub user_id: Thing,
    pub project_name: String,
    pub project_owner: Vec<ProjectOwner>,
    pub folder: Option<Vec<Folder>>,
    pub owner_name: Option<String>,
    pub card_number: Option<String>,
    pub card_brand: Option<CardBrand>,
    pub expired_month: Option<Vec<Month>>,
    pub expired_year: Option<i32>,
    pub safe_code: Option<String>,
    pub remarks: Option<String>,
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
