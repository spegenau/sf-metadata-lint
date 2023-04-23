use crate::metadata::DiscoveryOutcomeGoal::DiscoveryOutcomeGoal;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryGoalOutcome  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "fieldLabel")]
	pub field_label: String,
	#[serde(rename = "goal")]
	pub goal: DiscoveryOutcomeGoal,
	#[serde(rename = "mappedField")]
	pub mapped_field: Option<String>,
}
