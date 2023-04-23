use crate::metadata::CommunityBaseTemplate::CommunityBaseTemplate;
use crate::metadata::CommunityTemplateBundleInfo::CommunityTemplateBundleInfo;
use crate::metadata::CommunityTemplateCategory::CommunityTemplateCategory;
use crate::metadata::CommunityTemplatePageSetting::CommunityTemplatePageSetting;
use crate::metadata::NavigationLinkSet::NavigationLinkSet;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityTemplateDefinition  {
	#[serde(rename = "baseTemplate")]
	pub base_template: Option<CommunityBaseTemplate>,
	#[serde(rename = "bundlesInfo")]
	pub bundles_info: Option<Vec<CommunityTemplateBundleInfo>>,
	#[serde(rename = "category")]
	pub category: CommunityTemplateCategory,
	#[serde(rename = "defaultBrandingSet")]
	pub default_branding_set: Option<String>,
	#[serde(rename = "defaultThemeDefinition")]
	pub default_theme_definition: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "enableExtendedCleanUpOnDelete")]
	pub enable_extended_clean_up_on_delete: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "navigationLinkSet")]
	pub navigation_link_set: Option<Vec<NavigationLinkSet>>,
	#[serde(rename = "pageSetting")]
	pub page_setting: Option<Vec<CommunityTemplatePageSetting>>,
	#[serde(rename = "publisher")]
	pub publisher: Option<String>,
}
