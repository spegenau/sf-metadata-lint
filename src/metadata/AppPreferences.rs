use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppPreferences  {
	#[serde(rename = "enableCustomizeMyTabs")]
	pub enable_customize_my_tabs: bool,
	#[serde(rename = "enableKeyboardShortcuts")]
	pub enable_keyboard_shortcuts: bool,
	#[serde(rename = "enableListViewHover")]
	pub enable_list_view_hover: bool,
	#[serde(rename = "enableListViewReskin")]
	pub enable_list_view_reskin: bool,
	#[serde(rename = "enableMultiMonitorComponents")]
	pub enable_multi_monitor_components: bool,
	#[serde(rename = "enablePinTabs")]
	pub enable_pin_tabs: bool,
	#[serde(rename = "enableTabHover")]
	pub enable_tab_hover: bool,
	#[serde(rename = "enableTabLimits")]
	pub enable_tab_limits: bool,
	#[serde(rename = "saveUserSessions")]
	pub save_user_sessions: bool,
}
