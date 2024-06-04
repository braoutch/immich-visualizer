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
pub struct SystemConfigDto {
    #[serde(rename = "ffmpeg")]
    pub ffmpeg: Box<models::SystemConfigFFmpegDto>,
    #[serde(rename = "image")]
    pub image: Box<models::SystemConfigImageDto>,
    #[serde(rename = "job")]
    pub job: Box<models::SystemConfigJobDto>,
    #[serde(rename = "library")]
    pub library: Box<models::SystemConfigLibraryDto>,
    #[serde(rename = "logging")]
    pub logging: Box<models::SystemConfigLoggingDto>,
    #[serde(rename = "machineLearning")]
    pub machine_learning: Box<models::SystemConfigMachineLearningDto>,
    #[serde(rename = "map")]
    pub map: Box<models::SystemConfigMapDto>,
    #[serde(rename = "newVersionCheck")]
    pub new_version_check: Box<models::SystemConfigNewVersionCheckDto>,
    #[serde(rename = "notifications")]
    pub notifications: Box<models::SystemConfigNotificationsDto>,
    #[serde(rename = "oauth")]
    pub oauth: Box<models::SystemConfigOAuthDto>,
    #[serde(rename = "passwordLogin")]
    pub password_login: Box<models::SystemConfigPasswordLoginDto>,
    #[serde(rename = "reverseGeocoding")]
    pub reverse_geocoding: Box<models::SystemConfigReverseGeocodingDto>,
    #[serde(rename = "server")]
    pub server: Box<models::SystemConfigServerDto>,
    #[serde(rename = "storageTemplate")]
    pub storage_template: Box<models::SystemConfigStorageTemplateDto>,
    #[serde(rename = "theme")]
    pub theme: Box<models::SystemConfigThemeDto>,
    #[serde(rename = "trash")]
    pub trash: Box<models::SystemConfigTrashDto>,
    #[serde(rename = "user")]
    pub user: Box<models::SystemConfigUserDto>,
}

impl SystemConfigDto {
    pub fn new(ffmpeg: models::SystemConfigFFmpegDto, image: models::SystemConfigImageDto, job: models::SystemConfigJobDto, library: models::SystemConfigLibraryDto, logging: models::SystemConfigLoggingDto, machine_learning: models::SystemConfigMachineLearningDto, map: models::SystemConfigMapDto, new_version_check: models::SystemConfigNewVersionCheckDto, notifications: models::SystemConfigNotificationsDto, oauth: models::SystemConfigOAuthDto, password_login: models::SystemConfigPasswordLoginDto, reverse_geocoding: models::SystemConfigReverseGeocodingDto, server: models::SystemConfigServerDto, storage_template: models::SystemConfigStorageTemplateDto, theme: models::SystemConfigThemeDto, trash: models::SystemConfigTrashDto, user: models::SystemConfigUserDto) -> SystemConfigDto {
        SystemConfigDto {
            ffmpeg: Box::new(ffmpeg),
            image: Box::new(image),
            job: Box::new(job),
            library: Box::new(library),
            logging: Box::new(logging),
            machine_learning: Box::new(machine_learning),
            map: Box::new(map),
            new_version_check: Box::new(new_version_check),
            notifications: Box::new(notifications),
            oauth: Box::new(oauth),
            password_login: Box::new(password_login),
            reverse_geocoding: Box::new(reverse_geocoding),
            server: Box::new(server),
            storage_template: Box::new(storage_template),
            theme: Box::new(theme),
            trash: Box::new(trash),
            user: Box::new(user),
        }
    }
}

