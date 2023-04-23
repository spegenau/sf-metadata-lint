use crate::metadata::AppActionOverride::AppActionOverride;
use crate::metadata::AppBrand::AppBrand;
use crate::metadata::AppPreferences::AppPreferences;
use crate::metadata::AppProfileActionOverride::AppProfileActionOverride;
use crate::metadata::AppWorkspaceConfig::AppWorkspaceConfig;
use crate::metadata::FormFactor::FormFactor;
use crate::metadata::NavType::NavType;
use crate::metadata::ServiceCloudConsoleConfig::ServiceCloudConsoleConfig;
use crate::metadata::UiType::UiType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomApplication  {
	#[serde(rename = "actionOverrides")]
	pub action_overrides: Option<Vec<AppActionOverride>>,
	#[serde(rename = "brand")]
	pub brand: Option<AppBrand>,
	#[serde(rename = "consoleConfig")]
	pub console_config: Option<ServiceCloudConsoleConfig>,
	#[serde(rename = "defaultLandingTab")]
	pub default_landing_tab: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "formFactors")]
	pub form_factors: Option<Vec<FormFactor>>,
	#[serde(rename = "isNavAutoTempTabsDisabled")]
	pub is_nav_auto_temp_tabs_disabled: Option<bool>,
	#[serde(rename = "isNavPersonalizationDisabled")]
	pub is_nav_personalization_disabled: Option<bool>,
	#[serde(rename = "isNavTabPersistenceDisabled")]
	pub is_nav_tab_persistence_disabled: Option<bool>,
	#[serde(rename = "isServiceCloudConsole")]
	pub is_service_cloud_console: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "logo")]
	pub logo: Option<String>,
	#[serde(rename = "navType")]
	pub nav_type: Option<NavType>,
	#[serde(rename = "preferences")]
	pub preferences: Option<AppPreferences>,
	#[serde(rename = "profileActionOverrides")]
	pub profile_action_overrides: Option<Vec<AppProfileActionOverride>>,
	#[serde(rename = "setupExperience")]
	pub setup_experience: Option<String>,
	#[serde(rename = "subscriberTabs")]
	pub subscriber_tabs: Option<Vec<String>>,
	#[serde(rename = "tabs")]
	pub tabs: Option<Vec<String>>,
	#[serde(rename = "uiType")]
	pub ui_type: Option<UiType>,
	#[serde(rename = "utilityBar")]
	pub utility_bar: Option<String>,
	#[serde(rename = "workspaceConfig")]
	pub workspace_config: Option<AppWorkspaceConfig>,
}
