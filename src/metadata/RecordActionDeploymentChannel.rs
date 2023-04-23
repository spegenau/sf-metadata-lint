use crate::metadata::ChannelSource::ChannelSource;
use crate::metadata::RecordActionDefaultItem::RecordActionDefaultItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordActionDeploymentChannel  {
	#[serde(rename = "channel")]
	pub channel: ChannelSource,
	#[serde(rename = "channelItems")]
	pub channel_items: Option<Vec<RecordActionDefaultItem>>,
	#[serde(rename = "isAutopopEnabled")]
	pub is_autopop_enabled: Option<bool>,
}
