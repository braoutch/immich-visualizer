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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetMediaSize {
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "thumbnail")]
    Thumbnail,

}

impl std::fmt::Display for AssetMediaSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Preview => write!(f, "preview"),
            Self::Thumbnail => write!(f, "thumbnail"),
        }
    }
}

impl Default for AssetMediaSize {
    fn default() -> AssetMediaSize {
        Self::Preview
    }
}

