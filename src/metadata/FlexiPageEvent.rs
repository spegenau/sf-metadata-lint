use crate::metadata::FlexiPageEventSourceProperty::FlexiPageEventSourceProperty;
use crate::metadata::FlexiPageEventTarget::FlexiPageEventTarget;
use crate::metadata::FlexipageEventSourceTypeEnum::FlexipageEventSourceTypeEnum;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageEvent  {
	#[serde(rename = "sourceName")]
	pub source_name: String,
	#[serde(rename = "sourceProperties")]
	pub source_properties: Option<Vec<FlexiPageEventSourceProperty>>,
	#[serde(rename = "sourceType")]
	pub source_type: FlexipageEventSourceTypeEnum,
	#[serde(rename = "targets")]
	pub targets: Option<Vec<FlexiPageEventTarget>>,
}
