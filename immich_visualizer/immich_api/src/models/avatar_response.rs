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
pub struct AvatarResponse {
    #[serde(rename = "color")]
    pub color: models::UserAvatarColor,
}

impl AvatarResponse {
    pub fn new(color: models::UserAvatarColor) -> AvatarResponse {
        AvatarResponse {
            color,
        }
    }
}

