use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeUnionBase  {
	#[serde(rename = "limit")]
	pub limit: Option<i32>,
}
