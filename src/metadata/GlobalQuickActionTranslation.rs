use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GlobalQuickActionTranslation  {
	#[serde(rename = "aspect")]
	pub aspect: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
}
