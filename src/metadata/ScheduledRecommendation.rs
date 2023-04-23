use crate::metadata::ScheduledRecommendationDetail::ScheduledRecommendationDetail;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ScheduledRecommendation  {
	#[serde(rename = "scheduledRecommendationDetails")]
	pub scheduled_recommendation_details: Option<Vec<ScheduledRecommendationDetail>>,
}
