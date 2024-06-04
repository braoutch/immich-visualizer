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
pub struct SystemConfigSmtpDto {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    #[serde(rename = "transport")]
    pub transport: Box<models::SystemConfigSmtpTransportDto>,
}

impl SystemConfigSmtpDto {
    pub fn new(enabled: bool, from: String, reply_to: String, transport: models::SystemConfigSmtpTransportDto) -> SystemConfigSmtpDto {
        SystemConfigSmtpDto {
            enabled,
            from,
            reply_to,
            transport: Box::new(transport),
        }
    }
}

