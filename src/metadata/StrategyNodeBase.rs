use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeBase  {
	#[serde(rename = "childNode")]
	pub child_node: Option<Vec<String>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
}
