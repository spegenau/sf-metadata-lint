use crate::metadata::DiscoveryModelFieldType::DiscoveryModelFieldType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryModelField  {
	#[serde(rename = "isDisparateImpact")]
	pub is_disparate_impact: Option<bool>,
	#[serde(rename = "isSensitive")]
	pub is_sensitive: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "type")]
	pub _type: DiscoveryModelFieldType,
	#[serde(rename = "values")]
	pub values: Option<Vec<String>>,
}
