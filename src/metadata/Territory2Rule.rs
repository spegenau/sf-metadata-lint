use crate::metadata::Territory2RuleItem::Territory2RuleItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2Rule  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "objectType")]
	pub object_type: String,
	#[serde(rename = "ruleItems")]
	pub rule_items: Option<Vec<Territory2RuleItem>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
