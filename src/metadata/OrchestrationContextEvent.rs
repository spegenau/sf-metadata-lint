use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrchestrationContextEvent  {
	#[serde(rename = "eventType")]
	pub event_type: String,
	#[serde(rename = "orchestrationEvent")]
	pub orchestration_event: String,
	#[serde(rename = "platformEvent")]
	pub platform_event: String,
	#[serde(rename = "platformEventPrimaryKey")]
	pub platform_event_primary_key: String,
}
