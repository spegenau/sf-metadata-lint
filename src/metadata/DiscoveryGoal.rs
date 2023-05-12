use crate::metadata::DiscoveryDeployedModel::DiscoveryDeployedModel;
use crate::metadata::DiscoveryFilter::DiscoveryFilter;
use crate::metadata::DiscoveryGoalOutcome::DiscoveryGoalOutcome;
use crate::metadata::DiscoveryModelCard::DiscoveryModelCard;
use crate::metadata::DiscoveryPredictionType::DiscoveryPredictionType;
use crate::metadata::DiscoveryPushbackType::DiscoveryPushbackType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryGoal  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "deployedModels")]
	pub deployed_models: Option<Vec<DiscoveryDeployedModel>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "modelCards")]
	pub model_cards: Option<Vec<DiscoveryModelCard>>,
	#[serde(rename = "outcome")]
	pub outcome: DiscoveryGoalOutcome,
	#[serde(rename = "predictionType")]
	pub prediction_type: DiscoveryPredictionType,
	#[serde(rename = "pushbackField")]
	pub pushback_field: Option<String>,
	#[serde(rename = "pushbackType")]
	pub pushback_type: Option<DiscoveryPushbackType>,
	#[serde(rename = "subscribedEntity")]
	pub subscribed_entity: Option<String>,
	#[serde(rename = "terminalStateFilters")]
	pub terminal_state_filters: Option<Vec<DiscoveryFilter>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
