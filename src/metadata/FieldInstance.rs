use crate::metadata::FieldInstanceProperty::FieldInstanceProperty;
use crate::metadata::UiFormulaRule::UiFormulaRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldInstance  {
	#[serde(rename = "fieldInstanceProperties")]
	pub field_instance_properties: Option<Vec<FieldInstanceProperty>>,
	#[serde(rename = "fieldItem")]
	pub field_item: String,
	#[serde(rename = "identifier")]
	pub identifier: Option<String>,
	#[serde(rename = "visibilityRule")]
	pub visibility_rule: Option<UiFormulaRule>,
}
