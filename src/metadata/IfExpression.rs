use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IfExpression  {
	#[serde(rename = "childName")]
	pub child_name: String,
	#[serde(rename = "expression")]
	pub expression: String,
}
