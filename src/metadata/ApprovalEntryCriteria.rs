use crate::metadata::FilterItem::FilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalEntryCriteria  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<FilterItem>>,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
}
