use crate::metadata::StrategyAction::StrategyAction;
use crate::metadata::StrategyNodeExclusive::StrategyNodeExclusive;
use crate::metadata::StrategyNodeFilter::StrategyNodeFilter;
use crate::metadata::StrategyNodeIf::StrategyNodeIf;
use crate::metadata::StrategyNodeInvocableAction::StrategyNodeInvocableAction;
use crate::metadata::StrategyNodeMap::StrategyNodeMap;
use crate::metadata::StrategyNodeRecommendationLimit::StrategyNodeRecommendationLimit;
use crate::metadata::StrategyNodeRecommendationLoad::StrategyNodeRecommendationLoad;
use crate::metadata::StrategyNodeSort::StrategyNodeSort;
use crate::metadata::StrategyNodeUnion::StrategyNodeUnion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationStrategy  {
	#[serde(rename = "actionContext")]
	pub action_context: Option<Vec<StrategyAction>>,
	#[serde(rename = "contextRecordType")]
	pub context_record_type: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "filter")]
	pub filter: Option<Vec<StrategyNodeFilter>>,
	#[serde(rename = "if")]
	pub _if: Option<Vec<StrategyNodeIf>>,
	#[serde(rename = "invocableAction")]
	pub invocable_action: Option<Vec<StrategyNodeInvocableAction>>,
	#[serde(rename = "isTemplate")]
	pub is_template: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "map")]
	pub map: Option<Vec<StrategyNodeMap>>,
	#[serde(rename = "mutuallyExclusive")]
	pub mutually_exclusive: Option<Vec<StrategyNodeExclusive>>,
	#[serde(rename = "onBehalfOfExpression")]
	pub on_behalf_of_expression: Option<String>,
	#[serde(rename = "recommendationLimit")]
	pub recommendation_limit: Option<Vec<StrategyNodeRecommendationLimit>>,
	#[serde(rename = "recommendationLoad")]
	pub recommendation_load: Option<Vec<StrategyNodeRecommendationLoad>>,
	#[serde(rename = "sort")]
	pub sort: Option<Vec<StrategyNodeSort>>,
	#[serde(rename = "union")]
	pub union: Option<Vec<StrategyNodeUnion>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
