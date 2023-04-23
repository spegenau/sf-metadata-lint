use crate::metadata::ActionEmailSenderType::ActionEmailSenderType;
use crate::metadata::WorkflowEmailRecipient::WorkflowEmailRecipient;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowAlert  {
	#[serde(rename = "ccEmails")]
	pub cc_emails: Option<Vec<String>>,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "protected")]
	pub protected: bool,
	#[serde(rename = "recipients")]
	pub recipients: Option<Vec<WorkflowEmailRecipient>>,
	#[serde(rename = "senderAddress")]
	pub sender_address: Option<String>,
	#[serde(rename = "senderType")]
	pub sender_type: Option<ActionEmailSenderType>,
	#[serde(rename = "template")]
	pub template: String,
}
