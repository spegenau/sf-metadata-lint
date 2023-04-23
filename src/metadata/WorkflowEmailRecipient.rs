use crate::metadata::ActionEmailRecipientTypes::ActionEmailRecipientTypes;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowEmailRecipient  {
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "recipient")]
	pub recipient: Option<String>,
	#[serde(rename = "type")]
	pub _type: ActionEmailRecipientTypes,
}
