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
pub struct DownloadInfoDto {
    #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<uuid::Uuid>,
    #[serde(rename = "archiveSize", skip_serializing_if = "Option::is_none")]
    pub archive_size: Option<i32>,
    #[serde(rename = "assetIds", skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl DownloadInfoDto {
    pub fn new() -> DownloadInfoDto {
        DownloadInfoDto {
            album_id: None,
            archive_size: None,
            asset_ids: None,
            user_id: None,
        }
    }
}

