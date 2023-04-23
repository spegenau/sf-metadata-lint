use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SalesWorkQueueSettings  {
	#[serde(rename = "featureName")]
	pub feature_name: String,
	#[serde(rename = "targetEntity")]
	pub target_entity: String,
	#[serde(rename = "targetField")]
	pub target_field: String,
}
