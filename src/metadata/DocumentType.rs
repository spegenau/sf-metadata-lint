use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DocumentType  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
