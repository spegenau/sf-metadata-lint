use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunityListFieldsLabelMapping  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "label")]
	pub label: String,
}
