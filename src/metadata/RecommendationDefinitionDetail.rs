use crate::metadata::ScheduledRecommendation::ScheduledRecommendation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationDefinitionDetail  {
	#[serde(rename = "actionUrl")]
	pub action_url: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "linkText")]
	pub link_text: Option<String>,
	#[serde(rename = "scheduledRecommendations")]
	pub scheduled_recommendations: Option<ScheduledRecommendation>,
	#[serde(rename = "setupName")]
	pub setup_name: Option<String>,
	#[serde(rename = "title")]
	pub title: Option<String>,
}
