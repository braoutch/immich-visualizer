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
pub struct AvatarUpdate {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<models::UserAvatarColor>,
}

impl AvatarUpdate {
    pub fn new() -> AvatarUpdate {
        AvatarUpdate {
            color: None,
        }
    }
}

