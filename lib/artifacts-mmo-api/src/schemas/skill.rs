use serde::Deserialize;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_resources_resources__code__get>
#[derive(Debug, Clone, Deserialize)]
pub enum SkillSchema {
    Mining,
    Woodcutting,
    Fishing,
}
