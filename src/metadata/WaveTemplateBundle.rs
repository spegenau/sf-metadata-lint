use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveTemplateBundle  {
	#[serde(rename = "assetIcon")]
	pub asset_icon: Option<String>,
	#[serde(rename = "assetVersion")]
	pub asset_version: Option<f32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "templateType")]
	pub template_type: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
