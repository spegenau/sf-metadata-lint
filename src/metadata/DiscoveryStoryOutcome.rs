use crate::metadata::DiscoveryStoryOutcomeGoal::DiscoveryStoryOutcomeGoal;
use crate::metadata::DiscoveryStoryOutcomeType::DiscoveryStoryOutcomeType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryStoryOutcome  {
	#[serde(rename = "failureValue")]
	pub failure_value: Option<String>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "goal")]
	pub goal: DiscoveryStoryOutcomeGoal,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "successValue")]
	pub success_value: Option<String>,
	#[serde(rename = "type")]
	pub _type: DiscoveryStoryOutcomeType,
}
