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
pub struct JobSettingsDto {
    #[serde(rename = "concurrency")]
    pub concurrency: i32,
}

impl JobSettingsDto {
    pub fn new(concurrency: i32) -> JobSettingsDto {
        JobSettingsDto {
            concurrency,
        }
    }
}

