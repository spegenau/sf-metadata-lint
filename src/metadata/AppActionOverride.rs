use crate::metadata::ActionOverrideType::ActionOverrideType;
use crate::metadata::FormFactor::FormFactor;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppActionOverride  {
	#[serde(rename = "pageOrSobjectType")]
	pub page_or_sobject_type: String,
	#[serde(rename = "actionName")]
	pub action_name: Option<String>,
	#[serde(rename = "comment")]
	pub comment: Option<String>,
	#[serde(rename = "content")]
	pub content: Option<String>,
	#[serde(rename = "formFactor")]
	pub form_factor: Option<FormFactor>,
	#[serde(rename = "skipRecordTypeSelect")]
	pub skip_record_type_select: Option<bool>,
	#[serde(rename = "type")]
	pub _type: Option<ActionOverrideType>,
}
