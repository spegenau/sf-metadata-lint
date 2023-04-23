use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordActionDeploymentContext  {
	#[serde(rename = "entityName")]
	pub entity_name: String,
	#[serde(rename = "recommendationStrategy")]
	pub recommendation_strategy: Option<String>,
}
