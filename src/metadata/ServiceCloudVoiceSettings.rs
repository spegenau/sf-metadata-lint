use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceCloudVoiceSettings  {
	#[serde(rename = "enableAmazonQueueManagement")]
	pub enable_amazon_queue_management: Option<bool>,
	#[serde(rename = "enableDefaultChannelForSCV")]
	pub enable_default_channel_for_scv: Option<bool>,
	#[serde(rename = "enableEndUserForSCV")]
	pub enable_end_user_for_scv: Option<bool>,
	#[serde(rename = "enableOmniCapacityForSCV")]
	pub enable_omni_capacity_for_scv: Option<bool>,
	#[serde(rename = "enablePTQueueManagement")]
	pub enable_pt_queue_management: Option<bool>,
	#[serde(rename = "enableSCVBYOT")]
	pub enable_scvbyot: Option<bool>,
	#[serde(rename = "enableSCVExternalTelephony")]
	pub enable_scv_external_telephony: Option<bool>,
	#[serde(rename = "enableServiceCloudVoice")]
	pub enable_service_cloud_voice: Option<bool>,
	#[serde(rename = "enableVoiceInGovCloudOptIn")]
	pub enable_voice_in_gov_cloud_opt_in: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
