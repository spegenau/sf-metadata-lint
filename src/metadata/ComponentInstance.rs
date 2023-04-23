use crate::metadata::ComponentInstanceProperty::ComponentInstanceProperty;
use crate::metadata::ComponentInstanceType::ComponentInstanceType;
use crate::metadata::UiFormulaRule::UiFormulaRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ComponentInstance  {
	#[serde(rename = "componentInstanceProperties")]
	pub component_instance_properties: Option<Vec<ComponentInstanceProperty>>,
	#[serde(rename = "componentName")]
	pub component_name: String,
	#[serde(rename = "componentType")]
	pub component_type: Option<ComponentInstanceType>,
	#[serde(rename = "identifier")]
	pub identifier: Option<String>,
	#[serde(rename = "visibilityRule")]
	pub visibility_rule: Option<UiFormulaRule>,
}
