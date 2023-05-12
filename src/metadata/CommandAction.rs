use crate::metadata::CommandActionIntent::CommandActionIntent;
use crate::metadata::CommandActionParam::CommandActionParam;
use crate::metadata::CommandActionResponse::CommandActionResponse;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommandAction  {
	#[serde(rename = "actionType")]
	pub action_type: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "intents")]
	pub intents: Option<Vec<CommandActionIntent>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "parameters")]
	pub parameters: Option<Vec<CommandActionParam>>,
	#[serde(rename = "responseTemplates")]
	pub response_templates: Option<Vec<CommandActionResponse>>,
	#[serde(rename = "target")]
	pub target: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
