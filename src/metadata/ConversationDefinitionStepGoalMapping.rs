use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationDefinitionStepGoalMapping  {
	#[serde(rename = "goalName")]
	pub goal_name: String,
}
