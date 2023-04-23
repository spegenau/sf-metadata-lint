use crate::metadata::RecommendationConditionOperator::RecommendationConditionOperator;
use crate::metadata::RecommendationConditionValue::RecommendationConditionValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationLoadCondition  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "operator")]
	pub operator: RecommendationConditionOperator,
	#[serde(rename = "value")]
	pub value: RecommendationConditionValue,
}
