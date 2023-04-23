use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkspaceMapping  {
	#[serde(rename = "fieldName")]
	pub field_name: Option<String>,
	#[serde(rename = "tab")]
	pub tab: String,
}
