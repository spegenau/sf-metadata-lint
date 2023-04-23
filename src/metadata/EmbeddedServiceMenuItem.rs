use crate::metadata::EmbeddedServiceChannelType::EmbeddedServiceChannelType;
use crate::metadata::EmbeddedServiceCustomLabel::EmbeddedServiceCustomLabel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceMenuItem  {
	#[serde(rename = "channel")]
	pub channel: Option<String>,
	#[serde(rename = "channelType")]
	pub channel_type: Option<EmbeddedServiceChannelType>,
	#[serde(rename = "customUrl")]
	pub custom_url: Option<String>,
	#[serde(rename = "displayOrder")]
	pub display_order: Option<i32>,
	#[serde(rename = "embeddedServiceCustomLabels")]
	pub embedded_service_custom_labels: Option<Vec<EmbeddedServiceCustomLabel>>,
	#[serde(rename = "iconUrl")]
	pub icon_url: Option<String>,
	#[serde(rename = "isDisplayedOnPageLoad")]
	pub is_displayed_on_page_load: bool,
	#[serde(rename = "itemName")]
	pub item_name: String,
	#[serde(rename = "osOptionsHideInIOS")]
	pub os_options_hide_in_ios: Option<bool>,
	#[serde(rename = "osOptionsHideInLinuxOS")]
	pub os_options_hide_in_linux_os: Option<bool>,
	#[serde(rename = "osOptionsHideInMacOS")]
	pub os_options_hide_in_mac_os: Option<bool>,
	#[serde(rename = "osOptionsHideInOtherOS")]
	pub os_options_hide_in_other_os: Option<bool>,
	#[serde(rename = "osOptionsHideInWindowsOS")]
	pub os_options_hide_in_windows_os: Option<bool>,
	#[serde(rename = "phoneNumber")]
	pub phone_number: Option<String>,
	#[serde(rename = "shouldOpenUrlInSameTab")]
	pub should_open_url_in_same_tab: Option<bool>,
}
