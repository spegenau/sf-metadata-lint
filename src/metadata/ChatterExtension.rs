use crate::metadata::ChatterExtensionType::ChatterExtensionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChatterExtension  {
	#[serde(rename = "compositionComponent")]
	pub composition_component: String,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "extensionName")]
	pub extension_name: String,
	#[serde(rename = "headerText")]
	pub header_text: Option<String>,
	#[serde(rename = "hoverText")]
	pub hover_text: Option<String>,
	#[serde(rename = "icon")]
	pub icon: String,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "renderComponent")]
	pub render_component: String,
	#[serde(rename = "type")]
	pub _type: ChatterExtensionType,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
