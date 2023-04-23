use crate::metadata::BrandingSetProperty::BrandingSetProperty;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BrandingSet  {
	#[serde(rename = "brandingSetProperty")]
	pub branding_set_property: Option<Vec<BrandingSetProperty>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "type")]
	pub _type: Option<String>,
}
