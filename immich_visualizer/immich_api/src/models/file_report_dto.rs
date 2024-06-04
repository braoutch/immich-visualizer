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
pub struct FileReportDto {
    #[serde(rename = "extras")]
    pub extras: Vec<String>,
    #[serde(rename = "orphans")]
    pub orphans: Vec<models::FileReportItemDto>,
}

impl FileReportDto {
    pub fn new(extras: Vec<String>, orphans: Vec<models::FileReportItemDto>) -> FileReportDto {
        FileReportDto {
            extras,
            orphans,
        }
    }
}

