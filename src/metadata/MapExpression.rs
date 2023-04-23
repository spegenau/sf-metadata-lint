use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MapExpression  {
	#[serde(rename = "expression")]
	pub expression: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "type")]
	pub _type: Option<String>,
}
