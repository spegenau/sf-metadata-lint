use crate::metadata::ExpFeedbackCollType::ExpFeedbackCollType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BusinessProcessFeedback  {
	#[serde(rename = "actionName")]
	pub action_name: String,
	#[serde(rename = "actionParam")]
	pub action_param: String,
	#[serde(rename = "actionType")]
	pub action_type: ExpFeedbackCollType,
}
