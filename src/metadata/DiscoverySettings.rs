use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoverySettings  {
	#[serde(rename = "enableEinsteinAnswersPref")]
	pub enable_einstein_answers_pref: Option<bool>,
	#[serde(rename = "enableEinsteinArticleRecommendations")]
	pub enable_einstein_article_recommendations: Option<bool>,
}
