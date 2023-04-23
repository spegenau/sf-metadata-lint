use crate::metadata::ChannelLayoutItem::ChannelLayoutItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChannelLayout  {
	#[serde(rename = "doesExcludeFieldLabels")]
	pub does_exclude_field_labels: Option<bool>,
	#[serde(rename = "doesExcludeFiles")]
	pub does_exclude_files: Option<bool>,
	#[serde(rename = "enabledChannels")]
	pub enabled_channels: Option<Vec<String>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "layoutItems")]
	pub layout_items: Option<Vec<ChannelLayoutItem>>,
	#[serde(rename = "recordType")]
	pub record_type: Option<String>,
}
