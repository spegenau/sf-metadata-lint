use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReferencedDashboard  {
	#[serde(rename = "application")]
	pub application: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "embedUrl")]
	pub embed_url: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "templateAssetSourceName")]
	pub template_asset_source_name: Option<String>,
	#[serde(rename = "visibility")]
	pub visibility: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
