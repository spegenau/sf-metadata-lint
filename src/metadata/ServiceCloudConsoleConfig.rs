use crate::metadata::AppComponentList::AppComponentList;
use crate::metadata::KeyboardShortcuts::KeyboardShortcuts;
use crate::metadata::ListPlacement::ListPlacement;
use crate::metadata::LiveAgentConfig::LiveAgentConfig;
use crate::metadata::PushNotification::PushNotification;
use crate::metadata::TabLimitConfig::TabLimitConfig;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceCloudConsoleConfig  {
	#[serde(rename = "componentList")]
	pub component_list: Option<AppComponentList>,
	#[serde(rename = "detailPageRefreshMethod")]
	pub detail_page_refresh_method: String,
	#[serde(rename = "footerColor")]
	pub footer_color: Option<String>,
	#[serde(rename = "headerColor")]
	pub header_color: Option<String>,
	#[serde(rename = "keyboardShortcuts")]
	pub keyboard_shortcuts: KeyboardShortcuts,
	#[serde(rename = "listPlacement")]
	pub list_placement: ListPlacement,
	#[serde(rename = "listRefreshMethod")]
	pub list_refresh_method: String,
	#[serde(rename = "liveAgentConfig")]
	pub live_agent_config: Option<LiveAgentConfig>,
	#[serde(rename = "primaryTabColor")]
	pub primary_tab_color: Option<String>,
	#[serde(rename = "pushNotifications")]
	pub push_notifications: Option<Vec<PushNotification>>,
	#[serde(rename = "tabLimitConfig")]
	pub tab_limit_config: Option<TabLimitConfig>,
	#[serde(rename = "whitelistedDomains")]
	pub whitelisted_domains: Option<Vec<String>>,
}
