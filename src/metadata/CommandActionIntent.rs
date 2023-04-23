use crate::metadata::CommandActionResponse::CommandActionResponse;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommandActionIntent  {
	#[serde(rename = "phrase")]
	pub phrase: String,
	#[serde(rename = "responseTemplates")]
	pub response_templates: Option<Vec<CommandActionResponse>>,
}
