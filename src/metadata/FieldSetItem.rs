use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldSetItem  {
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "isFieldManaged")]
	pub is_field_managed: Option<bool>,
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
}
