use crate::metadata::SendAction::SendAction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowSend  {
	#[serde(rename = "action")]
	pub action: SendAction,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "language")]
	pub language: Option<String>,
	#[serde(rename = "protected")]
	pub protected: bool,
}
