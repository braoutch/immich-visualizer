/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.105.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryResponseDto {
    #[serde(rename = "assets")]
    pub assets: Vec<models::AssetResponseDto>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "data")]
    pub data: Box<models::OnThisDayDto>,
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    #[serde(rename = "memoryAt")]
    pub memory_at: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "seenAt", skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl MemoryResponseDto {
    pub fn new(assets: Vec<models::AssetResponseDto>, created_at: String, data: models::OnThisDayDto, id: String, is_saved: bool, memory_at: String, owner_id: String, r#type: Type, updated_at: String) -> MemoryResponseDto {
        MemoryResponseDto {
            assets,
            created_at,
            data: Box::new(data),
            deleted_at: None,
            id,
            is_saved,
            memory_at,
            owner_id,
            seen_at: None,
            r#type,
            updated_at,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "on_this_day")]
    OnThisDay,
}

impl Default for Type {
    fn default() -> Type {
        Self::OnThisDay
    }
}

