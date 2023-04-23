use crate::metadata::PinnedAction::PinnedAction;
use crate::metadata::RecordActionType::RecordActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordActionDefaultItem  {
	#[serde(rename = "action")]
	pub action: String,
	#[serde(rename = "isMandatory")]
	pub is_mandatory: Option<bool>,
	#[serde(rename = "isUiRemoveHidden")]
	pub is_ui_remove_hidden: Option<bool>,
	#[serde(rename = "pinned")]
	pub pinned: PinnedAction,
	#[serde(rename = "position")]
	pub position: i32,
	#[serde(rename = "type")]
	pub _type: RecordActionType,
}
