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
pub struct UserAdminResponseDto {
    #[serde(rename = "avatarColor")]
    pub avatar_color: models::UserAvatarColor,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "deletedAt", deserialize_with = "Option::deserialize")]
    pub deleted_at: Option<String>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "oauthId")]
    pub oauth_id: String,
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    #[serde(rename = "quotaSizeInBytes", deserialize_with = "Option::deserialize")]
    pub quota_size_in_bytes: Option<i64>,
    #[serde(rename = "quotaUsageInBytes", deserialize_with = "Option::deserialize")]
    pub quota_usage_in_bytes: Option<i64>,
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "storageLabel", deserialize_with = "Option::deserialize")]
    pub storage_label: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl UserAdminResponseDto {
    pub fn new(avatar_color: models::UserAvatarColor, created_at: String, deleted_at: Option<String>, email: String, id: String, is_admin: bool, name: String, oauth_id: String, profile_image_path: String, quota_size_in_bytes: Option<i64>, quota_usage_in_bytes: Option<i64>, should_change_password: bool, status: models::UserStatus, storage_label: Option<String>, updated_at: String) -> UserAdminResponseDto {
        UserAdminResponseDto {
            avatar_color,
            created_at,
            deleted_at,
            email,
            id,
            is_admin,
            name,
            oauth_id,
            profile_image_path,
            quota_size_in_bytes,
            quota_usage_in_bytes,
            should_change_password,
            status,
            storage_label,
            updated_at,
        }
    }
}

