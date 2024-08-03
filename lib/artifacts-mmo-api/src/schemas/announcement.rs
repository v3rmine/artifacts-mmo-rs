use chrono::{DateTime, Utc};
use serde::Serialize;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_status__get>
#[derive(Debug, Clone, Serialize)]
pub struct AnnouncementSchema {
    message: String,
    created_at: DateTime<Utc>,
}
