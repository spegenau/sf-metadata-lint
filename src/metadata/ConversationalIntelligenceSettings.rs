use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationalIntelligenceSettings  {
	#[serde(rename = "enableCallCoaching")]
	pub enable_call_coaching: Option<bool>,
	#[serde(rename = "enableCallCoachingZoom")]
	pub enable_call_coaching_zoom: Option<bool>,
	#[serde(rename = "enableOpptyMatching")]
	pub enable_oppty_matching: Option<bool>,
	#[serde(rename = "enableUnifiedActivities")]
	pub enable_unified_activities: Option<bool>,
}
