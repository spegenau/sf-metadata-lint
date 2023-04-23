use crate::metadata::IfExpression::IfExpression;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeIf  {
	#[serde(rename = "childNodeExpression")]
	pub child_node_expression: Option<Vec<IfExpression>>,
	#[serde(rename = "onlyFirstMatch")]
	pub only_first_match: Option<bool>,
}
