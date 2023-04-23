use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationDefinitionChannelProvider  {
	#[serde(rename = "agentRequired")]
	pub agent_required: Option<bool>,
	#[serde(rename = "chatButtonName")]
	pub chat_button_name: String,
}
