use chrono::{DateTime, Utc};

use crate::domain::project::{AdditionalField, ProjectFolder, ProjectOwner};

pub enum Status {
    Husband,
    Wife,
    Sir,
    Madam,
    Doctor,
    Others,
}

pub struct ID {
    pub project_name: String,
    pub project_owner: Vec<ProjectOwner>,
    pub folder: Option<Vec<ProjectFolder>>,
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
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
