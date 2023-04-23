use crate::metadata::ActionOverride::ActionOverride;
use crate::metadata::Encoding::Encoding;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomTab  {
	#[serde(rename = "actionOverrides")]
	pub action_overrides: Option<Vec<ActionOverride>>,
	#[serde(rename = "auraComponent")]
	pub aura_component: Option<String>,
	#[serde(rename = "customObject")]
	pub custom_object: Option<bool>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "flexiPage")]
	pub flexi_page: Option<String>,
	#[serde(rename = "frameHeight")]
	pub frame_height: Option<i32>,
	#[serde(rename = "hasSidebar")]
	pub has_sidebar: Option<bool>,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "lwcComponent")]
	pub lwc_component: Option<String>,
	#[serde(rename = "motif")]
	pub motif: Option<String>,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "scontrol")]
	pub scontrol: Option<String>,
	#[serde(rename = "splashPageLink")]
	pub splash_page_link: Option<String>,
	#[serde(rename = "url")]
	pub url: Option<String>,
	#[serde(rename = "urlEncodingKey")]
	pub url_encoding_key: Option<Encoding>,
}
