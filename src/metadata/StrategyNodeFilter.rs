use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeFilter  {
	#[serde(rename = "expression")]
	pub expression: String,
}
