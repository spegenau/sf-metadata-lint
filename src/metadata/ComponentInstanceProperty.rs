use crate::metadata::ComponentInstancePropertyList::ComponentInstancePropertyList;
use crate::metadata::ComponentInstancePropertyTypeEnum::ComponentInstancePropertyTypeEnum;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ComponentInstanceProperty  {
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub _type: Option<ComponentInstancePropertyTypeEnum>,
	#[serde(rename = "value")]
	pub value: Option<String>,
	#[serde(rename = "valueList")]
	pub value_list: Option<ComponentInstancePropertyList>,
}
