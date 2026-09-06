use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiKeyData {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) last_used_at: Option<DateTime<Utc>>,
    pub(crate) expires_at: DateTime<Utc>,
    pub(crate) is_active: bool,
    pub(crate) created_at: DateTime<Utc>,
}