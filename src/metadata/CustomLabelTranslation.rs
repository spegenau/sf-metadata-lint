use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomLabelTranslation  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
}
