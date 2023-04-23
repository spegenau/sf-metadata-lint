use crate::metadata::StrategyNodeSortField::StrategyNodeSortField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeSort  {
	#[serde(rename = "field")]
	pub field: Option<Vec<StrategyNodeSortField>>,
}
