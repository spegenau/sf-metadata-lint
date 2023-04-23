use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityCustomThemeLayoutType  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
}
