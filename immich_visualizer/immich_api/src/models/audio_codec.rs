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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudioCodec {
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "libopus")]
    Libopus,

}

impl std::fmt::Display for AudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mp3 => write!(f, "mp3"),
            Self::Aac => write!(f, "aac"),
            Self::Libopus => write!(f, "libopus"),
        }
    }
}

impl Default for AudioCodec {
    fn default() -> AudioCodec {
        Self::Mp3
    }
}

