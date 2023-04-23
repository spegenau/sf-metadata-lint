use crate::metadata::FilterItem::FilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingCriteriaRule  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<FilterItem>>,
	#[serde(rename = "includeRecordsOwnedByAll")]
	pub include_records_owned_by_all: bool,
}
