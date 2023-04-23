use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StandardFieldTranslation  {
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
}
