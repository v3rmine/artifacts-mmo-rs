use serde::{Deserialize, Serialize};
use strum::Display;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_resources_resources__code__get>
#[derive(Debug, Clone, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SkillSchema {
    Mining,
    Woodcutting,
    Fishing,
}
