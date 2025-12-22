use chrono::{DateTime, Utc};

use crate::domain::project::{AdditionalField, ProjectFolder, ProjectOwner};

pub struct Note {
    pub project_name: String,
    pub project_owner: Vec<ProjectOwner>,
    pub folder: Option<Vec<ProjectFolder>>,
    pub remarks: Option<String>,
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}
