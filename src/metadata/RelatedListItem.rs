use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelatedListItem  {
	#[serde(rename = "customButtons")]
	pub custom_buttons: Option<Vec<String>>,
	#[serde(rename = "excludeButtons")]
	pub exclude_buttons: Option<Vec<String>>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "relatedList")]
	pub related_list: String,
	#[serde(rename = "sortField")]
	pub sort_field: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<SortOrder>,
}
