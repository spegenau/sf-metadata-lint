use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldOverride  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "literalValue")]
	pub literal_value: Option<String>,
}
