use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BrandingSetProperty  {
	#[serde(rename = "propertyName")]
	pub property_name: String,
	#[serde(rename = "propertyValue")]
	pub property_value: Option<String>,
}
