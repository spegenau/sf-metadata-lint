use crate::metadata::FlowInputParameterTranslation::FlowInputParameterTranslation;
use crate::metadata::FlowInputValidationRuleTranslation::FlowInputValidationRuleTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreenFieldTranslation  {
	#[serde(rename = "fieldText")]
	pub field_text: Option<String>,
	#[serde(rename = "helpText")]
	pub help_text: Option<String>,
	#[serde(rename = "inputParameters")]
	pub input_parameters: Option<Vec<FlowInputParameterTranslation>>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "validationRule")]
	pub validation_rule: Option<FlowInputValidationRuleTranslation>,
}
