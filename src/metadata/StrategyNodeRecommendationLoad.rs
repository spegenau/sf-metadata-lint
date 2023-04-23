use crate::metadata::RecommendationLoadCondition::RecommendationLoadCondition;
use crate::metadata::StrategyNodeSortField::StrategyNodeSortField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeRecommendationLoad  {
	#[serde(rename = "condition")]
	pub condition: Option<Vec<RecommendationLoadCondition>>,
	#[serde(rename = "conditionLogic")]
	pub condition_logic: Option<String>,
	#[serde(rename = "object")]
	pub object: String,
	#[serde(rename = "sortField")]
	pub sort_field: Option<Vec<StrategyNodeSortField>>,
}
