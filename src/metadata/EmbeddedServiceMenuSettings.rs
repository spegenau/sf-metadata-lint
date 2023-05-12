use crate::metadata::EmbeddedServiceCustomLabel::EmbeddedServiceCustomLabel;
use crate::metadata::EmbeddedServiceCustomization::EmbeddedServiceCustomization;
use crate::metadata::EmbeddedServiceMenuItem::EmbeddedServiceMenuItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceMenuSettings  {
	#[serde(rename = "branding")]
	pub branding: Option<String>,
	#[serde(rename = "embeddedServiceCustomLabels")]
	pub embedded_service_custom_labels: Option<Vec<EmbeddedServiceCustomLabel>>,
	#[serde(rename = "embeddedServiceCustomizations")]
	pub embedded_service_customizations: Option<Vec<EmbeddedServiceCustomization>>,
	#[serde(rename = "embeddedServiceMenuItems")]
	pub embedded_service_menu_items: Option<Vec<EmbeddedServiceMenuItem>>,
	#[serde(rename = "isEnabled")]
	pub is_enabled: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "site")]
	pub site: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
