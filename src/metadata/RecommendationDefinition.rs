use crate::metadata::RecommendationDefinitionDetail::RecommendationDefinitionDetail;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationDefinition  {
	#[serde(rename = "recommendationDefinitionDetails")]
	pub recommendation_definition_details: Option<Vec<RecommendationDefinitionDetail>>,
}
