use crate::metadata::CommunityThemeLayoutType::CommunityThemeLayoutType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityThemeRouteOverride  {
	#[serde(rename = "customThemeLayoutType")]
	pub custom_theme_layout_type: Option<String>,
	#[serde(rename = "pageAttributes")]
	pub page_attributes: String,
	#[serde(rename = "pageType")]
	pub page_type: String,
	#[serde(rename = "themeLayoutType")]
	pub theme_layout_type: Option<CommunityThemeLayoutType>,
}
