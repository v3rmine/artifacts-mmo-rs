use serde::Deserialize;

use super::{CraftSchema, ItemEffectSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct ItemSchema {
    pub name: String,
    pub code: String,
    pub level: u32,
    pub r#type: String,
    pub subtype: String,
    pub description: String,
    pub effects: Vec<ItemEffectSchema>,
    pub craft: Option<CraftSchema>,
}
