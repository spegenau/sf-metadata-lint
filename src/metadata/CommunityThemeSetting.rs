use crate::metadata::CommunityThemeLayoutType::CommunityThemeLayoutType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityThemeSetting  {
	#[serde(rename = "customThemeLayoutType")]
	pub custom_theme_layout_type: Option<String>,
	#[serde(rename = "themeLayout")]
	pub theme_layout: String,
	#[serde(rename = "themeLayoutType")]
	pub theme_layout_type: Option<CommunityThemeLayoutType>,
}
