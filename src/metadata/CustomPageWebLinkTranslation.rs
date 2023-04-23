use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomPageWebLinkTranslation  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
}
