use crate::domain::{account::Account, card::Card, id::ID, note::Note, ssh::SSH};
use surrealdb::sql::Thing;

pub struct Vault {
    pub id: Thing,
    pub user_id: Thing,
    pub account_projects: Option<Vec<Account>>,
    pub card_projects: Option<Vec<Card>>,
    pub id_projects: Option<Vec<ID>>,
    pub note_projects: Option<Vec<Note>>,
    pub ssh_projects: Option<Vec<SSH>>,
}
