use crate::metadata::FilterItem::FilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LookupFilter  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "errorMessage")]
	pub error_message: Option<String>,
	#[serde(rename = "filterItems")]
	pub filter_items: Option<Vec<FilterItem>>,
	#[serde(rename = "infoMessage")]
	pub info_message: Option<String>,
	#[serde(rename = "isOptional")]
	pub is_optional: bool,
}
