use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LayoutSectionTranslation  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "section")]
	pub section: String,
}
