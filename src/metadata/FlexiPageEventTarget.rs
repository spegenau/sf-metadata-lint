use crate::metadata::FlexiPageEventPropertyMapping::FlexiPageEventPropertyMapping;
use crate::metadata::FlexiPageEventTargetProperty::FlexiPageEventTargetProperty;
use crate::metadata::FlexipageEventTargetTypeEnum::FlexipageEventTargetTypeEnum;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageEventTarget  {
	#[serde(rename = "mappings")]
	pub mappings: Option<Vec<FlexiPageEventPropertyMapping>>,
	#[serde(rename = "method")]
	pub method: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "properties")]
	pub properties: Option<Vec<FlexiPageEventTargetProperty>>,
	#[serde(rename = "type")]
	pub _type: FlexipageEventTargetTypeEnum,
}
