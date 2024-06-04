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
pub struct AlbumUserAddDto {
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<models::AlbumUserRole>,
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
}

impl AlbumUserAddDto {
    pub fn new(user_id: uuid::Uuid) -> AlbumUserAddDto {
        AlbumUserAddDto {
            role: None,
            user_id,
        }
    }
}

