use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityTemplatePageSetting  {
	#[serde(rename = "page")]
	pub page: String,
	#[serde(rename = "themeLayout")]
	pub theme_layout: String,
}
