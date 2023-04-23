use crate::metadata::FlowChoiceUserInput::FlowChoiceUserInput;
use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowChoice  {
	#[serde(rename = "choiceText")]
	pub choice_text: String,
	#[serde(rename = "dataType")]
	pub data_type: FlowDataType,
	#[serde(rename = "userInput")]
	pub user_input: Option<FlowChoiceUserInput>,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
