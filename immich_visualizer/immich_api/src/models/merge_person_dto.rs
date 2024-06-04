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
pub struct MergePersonDto {
    #[serde(rename = "ids")]
    pub ids: Vec<uuid::Uuid>,
}

impl MergePersonDto {
    pub fn new(ids: Vec<uuid::Uuid>) -> MergePersonDto {
        MergePersonDto {
            ids,
        }
    }
}

