use crate::metadata::FlexiPageRegionMode::FlexiPageRegionMode;
use crate::metadata::FlexiPageRegionType::FlexiPageRegionType;
use crate::metadata::ItemInstance::ItemInstance;
use crate::metadata::RegionFlagStatus::RegionFlagStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageRegion  {
	#[serde(rename = "appendable")]
	pub appendable: Option<RegionFlagStatus>,
	#[serde(rename = "itemInstances")]
	pub item_instances: Option<Vec<ItemInstance>>,
	#[serde(rename = "mode")]
	pub mode: Option<FlexiPageRegionMode>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "prependable")]
	pub prependable: Option<RegionFlagStatus>,
	#[serde(rename = "replaceable")]
	pub replaceable: Option<RegionFlagStatus>,
	#[serde(rename = "type")]
	pub _type: FlexiPageRegionType,
}
