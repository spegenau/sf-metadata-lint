use crate::metadata::DiscoveryFilterValueType::DiscoveryFilterValueType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryFilterValue  {
	#[serde(rename = "type")]
	pub _type: DiscoveryFilterValueType,
	#[serde(rename = "value")]
	pub value: String,
}
