use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailServerConfig {
    pub resend_from_email: String,
    pub resend_api_key: String,
}
