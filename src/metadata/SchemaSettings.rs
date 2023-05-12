use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SchemaSettings  {
	#[serde(rename = "enableAdvancedCMTSecurity")]
	pub enable_advanced_cmt_security: Option<bool>,
	#[serde(rename = "enableAdvancedCSSecurity")]
	pub enable_advanced_cs_security: Option<bool>,
	#[serde(rename = "enableListCustomSettingCreation")]
	pub enable_list_custom_setting_creation: Option<bool>,
	#[serde(rename = "enableSOSLOnCustomSettings")]
	pub enable_sosl_on_custom_settings: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
