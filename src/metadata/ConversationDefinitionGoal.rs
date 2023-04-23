use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationDefinitionGoal  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: String,
}
