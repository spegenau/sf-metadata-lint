use crate::metadata::DataCategory::DataCategory;
use crate::metadata::ObjectUsage::ObjectUsage;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DataCategoryGroup  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "dataCategory")]
	pub data_category: DataCategory,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "objectUsage")]
	pub object_usage: Option<ObjectUsage>,
}
