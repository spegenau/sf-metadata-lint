use crate::metadata::DevicePlatformType::DevicePlatformType;
use crate::metadata::DeviceType::DeviceType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppMobileDetailConfig  {
	#[serde(rename = "applicationBinaryFile")]
	pub application_binary_file: Option<String>,
	#[serde(rename = "applicationBinaryFileName")]
	pub application_binary_file_name: Option<String>,
	#[serde(rename = "applicationBundleIdentifier")]
	pub application_bundle_identifier: Option<String>,
	#[serde(rename = "applicationFileLength")]
	pub application_file_length: Option<i32>,
	#[serde(rename = "applicationIconFile")]
	pub application_icon_file: Option<String>,
	#[serde(rename = "applicationIconFileName")]
	pub application_icon_file_name: Option<String>,
	#[serde(rename = "applicationInstallUrl")]
	pub application_install_url: Option<String>,
	#[serde(rename = "devicePlatform")]
	pub device_platform: DevicePlatformType,
	#[serde(rename = "deviceType")]
	pub device_type: Option<DeviceType>,
	#[serde(rename = "minimumOsVersion")]
	pub minimum_os_version: Option<String>,
	#[serde(rename = "privateApp")]
	pub private_app: Option<bool>,
	#[serde(rename = "version")]
	pub version: String,
}
