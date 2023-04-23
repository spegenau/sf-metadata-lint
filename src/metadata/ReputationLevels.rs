use crate::metadata::ChatterAnswersReputationLevel::ChatterAnswersReputationLevel;
use crate::metadata::IdeaReputationLevel::IdeaReputationLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReputationLevels  {
	#[serde(rename = "chatterAnswersReputationLevels")]
	pub chatter_answers_reputation_levels: Option<Vec<ChatterAnswersReputationLevel>>,
	#[serde(rename = "ideaReputationLevels")]
	pub idea_reputation_levels: Option<Vec<IdeaReputationLevel>>,
}
