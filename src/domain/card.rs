use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::domain::{
    month::Month,
    project::{AdditionalField, ProjectFolder, ProjectOwner},
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
    pub project_name: String,
    pub project_owner: Vec<ProjectOwner>,
    pub folder: Option<Vec<ProjectFolder>>,
    pub owner_name: Option<String>,
    pub card_number: Option<String>,
    pub card_brand: Option<CardBrand>,
    pub expired_month: Option<HashMap<i32, Month>>,
    pub expired_year: Option<i32>,
    pub safe_code: Option<String>,
    pub remarks: Option<String>,
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
