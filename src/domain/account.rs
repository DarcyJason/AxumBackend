use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

use crate::domain::project::{AdditionalField, ProjectFolder, ProjectOwner};

pub struct Account {
    pub id: Thing,
    pub user_id: Thing,
    pub project_name: String,
    pub project_owner: Vec<ProjectOwner>,
    pub folder: Option<Vec<ProjectFolder>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub web_url: Option<String>,
    pub remarks: Option<String>,
    pub additional_fields: Option<Vec<AdditionalField>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
