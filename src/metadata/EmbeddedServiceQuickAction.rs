use crate::metadata::EmbeddedServiceQuickActionType::EmbeddedServiceQuickActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceQuickAction  {
	#[serde(rename = "embeddedServiceLiveAgent")]
	pub embedded_service_live_agent: String,
	#[serde(rename = "order")]
	pub order: i32,
	#[serde(rename = "quickActionDefinition")]
	pub quick_action_definition: String,
	#[serde(rename = "quickActionType")]
	pub quick_action_type: Option<EmbeddedServiceQuickActionType>,
}
