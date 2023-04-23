use crate::metadata::ValueSetValuesDefinition::ValueSetValuesDefinition;
use crate::metadata::ValueSettings::ValueSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValueSet  {
	#[serde(rename = "controllingField")]
	pub controlling_field: Option<String>,
	#[serde(rename = "restricted")]
	pub restricted: Option<bool>,
	#[serde(rename = "valueSetDefinition")]
	pub value_set_definition: Option<ValueSetValuesDefinition>,
	#[serde(rename = "valueSetName")]
	pub value_set_name: Option<String>,
	#[serde(rename = "valueSettings")]
	pub value_settings: Option<Vec<ValueSettings>>,
}
