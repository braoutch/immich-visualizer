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
pub struct ServerStorageResponseDto {
    #[serde(rename = "diskAvailable")]
    pub disk_available: String,
    #[serde(rename = "diskAvailableRaw")]
    pub disk_available_raw: i64,
    #[serde(rename = "diskSize")]
    pub disk_size: String,
    #[serde(rename = "diskSizeRaw")]
    pub disk_size_raw: i64,
    #[serde(rename = "diskUsagePercentage")]
    pub disk_usage_percentage: f32,
    #[serde(rename = "diskUse")]
    pub disk_use: String,
    #[serde(rename = "diskUseRaw")]
    pub disk_use_raw: i64,
}

impl ServerStorageResponseDto {
    pub fn new(disk_available: String, disk_available_raw: i64, disk_size: String, disk_size_raw: i64, disk_usage_percentage: f32, disk_use: String, disk_use_raw: i64) -> ServerStorageResponseDto {
        ServerStorageResponseDto {
            disk_available,
            disk_available_raw,
            disk_size,
            disk_size_raw,
            disk_usage_percentage,
            disk_use,
            disk_use_raw,
        }
    }
}

