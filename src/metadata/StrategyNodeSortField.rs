use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeSortField  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "nullsFirst")]
	pub nulls_first: Option<bool>,
	#[serde(rename = "order")]
	pub order: Option<SortOrder>,
}
