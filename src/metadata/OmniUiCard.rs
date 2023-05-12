use crate::metadata::OmniUiCardType::OmniUiCardType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniUiCard  {
	#[serde(rename = "authorName")]
	pub author_name: String,
	#[serde(rename = "clonedFromOmniUiCardKey")]
	pub cloned_from_omni_ui_card_key: Option<String>,
	#[serde(rename = "dataSourceConfig")]
	pub data_source_config: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "omniUiCardType")]
	pub omni_ui_card_type: OmniUiCardType,
	#[serde(rename = "overrideKey")]
	pub override_key: Option<String>,
	#[serde(rename = "propertySetConfig")]
	pub property_set_config: String,
	#[serde(rename = "sampleDataSourceResponse")]
	pub sample_data_source_response: Option<String>,
	#[serde(rename = "stylingConfiguration")]
	pub styling_configuration: Option<String>,
	#[serde(rename = "versionNumber")]
	pub version_number: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
