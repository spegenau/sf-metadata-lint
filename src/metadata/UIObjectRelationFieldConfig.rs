use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UIObjectRelationFieldConfig  {
	#[serde(rename = "displayLabel")]
	pub display_label: String,
	#[serde(rename = "queryText")]
	pub query_text: String,
	#[serde(rename = "rowOrder")]
	pub row_order: i32,
}
