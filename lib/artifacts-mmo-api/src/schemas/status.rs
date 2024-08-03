use serde::Serialize;

use super::{AnnouncementSchema, ResponseSchema};

pub type StatusResponse = ResponseSchema<StatusSchema>;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_status__get>
#[derive(Debug, Clone, Serialize)]
pub struct StatusSchema {
    status: String,
    version: String,
    characters_online: u32,
    announcements: Vec<AnnouncementSchema>,
    // REVIEW: not documented but might be dates
    last_wipe: String,
    next_wipe: String,
}
