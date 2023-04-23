use crate::metadata::FlowInputValidationRuleTranslation::FlowInputValidationRuleTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowChoiceUserInputTranslation  {
	#[serde(rename = "promptText")]
	pub prompt_text: Option<String>,
	#[serde(rename = "validationRule")]
	pub validation_rule: Option<FlowInputValidationRuleTranslation>,
}
