use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AIReplyRecommendationsSettings  {
	#[serde(rename = "enableAIReplyRecommendations")]
	pub enable_ai_reply_recommendations: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
