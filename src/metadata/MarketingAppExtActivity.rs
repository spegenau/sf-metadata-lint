use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MarketingAppExtActivity  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "endpointUrl")]
	pub endpoint_url: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "marketingAppExtension")]
	pub marketing_app_extension: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
