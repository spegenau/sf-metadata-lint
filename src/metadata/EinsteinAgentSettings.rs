use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EinsteinAgentSettings  {
	#[serde(rename = "einsteinAgentRecommendations")]
	pub einstein_agent_recommendations: Option<bool>,
	#[serde(rename = "reRunAttributeBasedRules")]
	pub re_run_attribute_based_rules: Option<bool>,
	#[serde(rename = "runAssignmentRules")]
	pub run_assignment_rules: Option<bool>,
}
