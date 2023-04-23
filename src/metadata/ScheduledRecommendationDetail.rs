use crate::metadata::RecommendationChannel::RecommendationChannel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ScheduledRecommendationDetail  {
	#[serde(rename = "channel")]
	pub channel: Option<RecommendationChannel>,
	#[serde(rename = "enabled")]
	pub enabled: Option<bool>,
	#[serde(rename = "rank")]
	pub rank: Option<i32>,
	#[serde(rename = "recommendationAudience")]
	pub recommendation_audience: Option<String>,
}
