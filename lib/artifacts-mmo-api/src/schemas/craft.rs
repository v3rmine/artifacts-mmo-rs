use serde::Deserialize;

use super::{SimpleItemSchema, SkillSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct CraftSchema {
    pub skill: SkillSchema,
    pub level: u32,
    pub items: Vec<SimpleItemSchema>,
    pub quantity: u32,
}
