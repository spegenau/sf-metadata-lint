use crate::metadata::FlowTransformValueAction::FlowTransformValueAction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTransformValue  {
	#[serde(rename = "inputReference")]
	pub input_reference: Option<String>,
	#[serde(rename = "transformValueActions")]
	pub transform_value_actions: Option<Vec<FlowTransformValueAction>>,
}
