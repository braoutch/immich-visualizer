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
pub struct PersonResponseDto {
    #[serde(rename = "birthDate", deserialize_with = "Option::deserialize")]
    pub birth_date: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "thumbnailPath")]
    pub thumbnail_path: String,
}

impl PersonResponseDto {
    pub fn new(birth_date: Option<String>, id: String, is_hidden: bool, name: String, thumbnail_path: String) -> PersonResponseDto {
        PersonResponseDto {
            birth_date,
            id,
            is_hidden,
            name,
            thumbnail_path,
        }
    }
}

