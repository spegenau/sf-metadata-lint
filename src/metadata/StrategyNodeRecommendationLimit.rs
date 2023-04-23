use crate::metadata::StrategyReactionType::StrategyReactionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeRecommendationLimit  {
	#[serde(rename = "filterMode")]
	pub filter_mode: Option<Vec<StrategyReactionType>>,
	#[serde(rename = "lookbackDuration")]
	pub lookback_duration: Option<i32>,
	#[serde(rename = "maxRecommendationCount")]
	pub max_recommendation_count: Option<i32>,
}
