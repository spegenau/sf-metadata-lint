use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ModeratedEntityField  {
	#[serde(rename = "entityName")]
	pub entity_name: String,
	#[serde(rename = "fieldName")]
	pub field_name: Option<String>,
	#[serde(rename = "keywordList")]
	pub keyword_list: Option<String>,
}
