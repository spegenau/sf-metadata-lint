use crate::metadata::MapExpression::MapExpression;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeMap  {
	#[serde(rename = "mapExpression")]
	pub map_expression: Option<Vec<MapExpression>>,
}
