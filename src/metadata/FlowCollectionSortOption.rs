use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCollectionSortOption  {
	#[serde(rename = "doesPutEmptyStringAndNullFirst")]
	pub does_put_empty_string_and_null_first: bool,
	#[serde(rename = "sortField")]
	pub sort_field: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: SortOrder,
}
