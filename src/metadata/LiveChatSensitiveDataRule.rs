use crate::metadata::SensitiveDataActionType::SensitiveDataActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatSensitiveDataRule  {
	#[serde(rename = "actionType")]
	pub action_type: SensitiveDataActionType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "enforceOn")]
	pub enforce_on: i32,
	#[serde(rename = "isEnabled")]
	pub is_enabled: bool,
	#[serde(rename = "pattern")]
	pub pattern: String,
	#[serde(rename = "priority")]
	pub priority: i32,
	#[serde(rename = "replacement")]
	pub replacement: Option<String>,
}
