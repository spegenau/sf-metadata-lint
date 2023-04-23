use crate::metadata::ConversationSystemDialogType::ConversationSystemDialogType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationSystemDialog  {
	#[serde(rename = "dialog")]
	pub dialog: String,
	#[serde(rename = "type")]
	pub _type: ConversationSystemDialogType,
}
