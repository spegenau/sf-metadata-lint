use crate::metadata::CommunityCustomThemeLayoutType::CommunityCustomThemeLayoutType;
use crate::metadata::CommunityThemeBundleInfo::CommunityThemeBundleInfo;
use crate::metadata::CommunityThemeRouteOverride::CommunityThemeRouteOverride;
use crate::metadata::CommunityThemeSetting::CommunityThemeSetting;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityThemeDefinition  {
	#[serde(rename = "bundlesInfo")]
	pub bundles_info: Option<Vec<CommunityThemeBundleInfo>>,
	#[serde(rename = "customThemeLayoutType")]
	pub custom_theme_layout_type: Option<Vec<CommunityCustomThemeLayoutType>>,
	#[serde(rename = "defaultBrandingSet")]
	pub default_branding_set: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "enableExtendedCleanUpOnDelete")]
	pub enable_extended_clean_up_on_delete: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "publisher")]
	pub publisher: Option<String>,
	#[serde(rename = "themeRouteOverride")]
	pub theme_route_override: Option<Vec<CommunityThemeRouteOverride>>,
	#[serde(rename = "themeSetting")]
	pub theme_setting: Option<Vec<CommunityThemeSetting>>,
}
