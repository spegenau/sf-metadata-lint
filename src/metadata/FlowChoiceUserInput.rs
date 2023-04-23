use crate::metadata::FlowInputValidationRule::FlowInputValidationRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowChoiceUserInput  {
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
	#[serde(rename = "promptText")]
	pub prompt_text: Option<String>,
	#[serde(rename = "validationRule")]
	pub validation_rule: Option<FlowInputValidationRule>,
}
