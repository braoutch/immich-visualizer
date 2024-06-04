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
pub struct BulkIdResponseDto {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "success")]
    pub success: bool,
}

impl BulkIdResponseDto {
    pub fn new(id: String, success: bool) -> BulkIdResponseDto {
        BulkIdResponseDto {
            error: None,
            id,
            success,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "no_permission")]
    NoPermission,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Error {
    fn default() -> Error {
        Self::Duplicate
    }
}

