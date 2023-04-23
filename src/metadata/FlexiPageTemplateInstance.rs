use crate::metadata::ComponentInstanceProperty::ComponentInstanceProperty;
use crate::metadata::ComponentInstanceType::ComponentInstanceType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageTemplateInstance  {
	#[serde(rename = "componentType")]
	pub component_type: Option<ComponentInstanceType>,
	#[serde(rename = "identifier")]
	pub identifier: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "properties")]
	pub properties: Option<Vec<ComponentInstanceProperty>>,
}
