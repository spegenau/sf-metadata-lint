use crate::metadata::RecommendationConditionValueType::RecommendationConditionValueType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationConditionValue  {
	#[serde(rename = "type")]
	pub _type: RecommendationConditionValueType,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
