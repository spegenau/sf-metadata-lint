use crate::metadata::RecommendationAudienceDetail::RecommendationAudienceDetail;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationAudience  {
	#[serde(rename = "recommendationAudienceDetails")]
	pub recommendation_audience_details: Option<Vec<RecommendationAudienceDetail>>,
}
