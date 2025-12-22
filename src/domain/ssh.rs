use chrono::{DateTime, Utc};

use crate::domain::project::{AdditionalField, ProjectFolder, ProjectOwner};

pub struct SSH {
    pub project_name: String,
    pub project_owner: Vec<ProjectOwner>,
    pub folder: Option<Vec<ProjectFolder>>,
    pub secret_key: Option<String>,
    pub public_key: Option<String>,
    pub fingerprint: Option<String>,
    pub remarks: Option<String>,
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
