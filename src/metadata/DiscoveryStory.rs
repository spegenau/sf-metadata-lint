use crate::metadata::DiscoveryStoryAutopilotStatus::DiscoveryStoryAutopilotStatus;
use crate::metadata::DiscoveryStoryOutcome::DiscoveryStoryOutcome;
use crate::metadata::DiscoveryStorySourceType::DiscoveryStorySourceType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryStory  {
	#[serde(rename = "application")]
	pub application: String,
	#[serde(rename = "autopilot")]
	pub autopilot: Option<DiscoveryStoryAutopilotStatus>,
	#[serde(rename = "classificationThreshold")]
	pub classification_threshold: Option<f32>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "outcome")]
	pub outcome: DiscoveryStoryOutcome,
	#[serde(rename = "sourceContainer")]
	pub source_container: String,
	#[serde(rename = "sourceType")]
	pub source_type: DiscoveryStorySourceType,
	#[serde(rename = "validationContainer")]
	pub validation_container: Option<String>,
}
