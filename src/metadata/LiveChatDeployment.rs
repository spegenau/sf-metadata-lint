use crate::metadata::LiveChatDeploymentDomainWhitelist::LiveChatDeploymentDomainWhitelist;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatDeployment  {
	#[serde(rename = "brandingImage")]
	pub branding_image: Option<String>,
	#[serde(rename = "connectionTimeoutDuration")]
	pub connection_timeout_duration: Option<i32>,
	#[serde(rename = "connectionWarningDuration")]
	pub connection_warning_duration: Option<i32>,
	#[serde(rename = "displayQueuePosition")]
	pub display_queue_position: Option<bool>,
	#[serde(rename = "domainWhiteList")]
	pub domain_white_list: Option<LiveChatDeploymentDomainWhitelist>,
	#[serde(rename = "enablePrechatApi")]
	pub enable_prechat_api: Option<bool>,
	#[serde(rename = "enableTranscriptSave")]
	pub enable_transcript_save: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mobileBrandingImage")]
	pub mobile_branding_image: Option<String>,
	#[serde(rename = "site")]
	pub site: Option<String>,
	#[serde(rename = "windowTitle")]
	pub window_title: String,
}
