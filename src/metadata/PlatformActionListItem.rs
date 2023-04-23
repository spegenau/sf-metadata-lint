use crate::metadata::PlatformActionType::PlatformActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformActionListItem  {
	#[serde(rename = "actionName")]
	pub action_name: String,
	#[serde(rename = "actionType")]
	pub action_type: PlatformActionType,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
	#[serde(rename = "subtype")]
	pub subtype: Option<String>,
}
