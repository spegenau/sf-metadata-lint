use crate::metadata::FlowInputValidationRule::FlowInputValidationRule;
use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowChoiceUserInput  {
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
	#[serde(rename = "promptText")]
	pub prompt_text: Option<String>,
	#[serde(rename = "validationRule")]
	pub validation_rule: Option<FlowInputValidationRule>,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
