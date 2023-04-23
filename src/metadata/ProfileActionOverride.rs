use crate::metadata::ActionOverrideType::ActionOverrideType;
use crate::metadata::FormFactor::FormFactor;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileActionOverride  {
	#[serde(rename = "actionName")]
	pub action_name: String,
	#[serde(rename = "content")]
	pub content: Option<String>,
	#[serde(rename = "formFactor")]
	pub form_factor: FormFactor,
	#[serde(rename = "pageOrSobjectType")]
	pub page_or_sobject_type: String,
	#[serde(rename = "recordType")]
	pub record_type: Option<String>,
	#[serde(rename = "type")]
	pub _type: ActionOverrideType,
}
