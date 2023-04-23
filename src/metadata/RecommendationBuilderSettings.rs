use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationBuilderSettings  {
	#[serde(rename = "enableErbEnabledPref")]
	pub enable_erb_enabled_pref: Option<bool>,
	#[serde(rename = "enableErbStartedPref")]
	pub enable_erb_started_pref: Option<bool>,
}
