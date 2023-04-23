use crate::metadata::FilterItem::FilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingGuestRule  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<FilterItem>>,
	#[serde(rename = "includeHVUOwnedRecords")]
	pub include_hvu_owned_records: bool,
}
