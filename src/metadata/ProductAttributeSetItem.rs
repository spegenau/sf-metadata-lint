use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProductAttributeSetItem  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "sequence")]
	pub sequence: i32,
}
