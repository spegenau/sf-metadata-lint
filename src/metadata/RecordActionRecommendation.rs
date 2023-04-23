use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordActionRecommendation  {
	#[serde(rename = "defaultStrategy")]
	pub default_strategy: Option<String>,
	#[serde(rename = "hasDescription")]
	pub has_description: bool,
	#[serde(rename = "hasImage")]
	pub has_image: bool,
	#[serde(rename = "hasRejectAction")]
	pub has_reject_action: bool,
	#[serde(rename = "hasTitle")]
	pub has_title: bool,
	#[serde(rename = "maxDisplayRecommendations")]
	pub max_display_recommendations: i32,
	#[serde(rename = "shouldLaunchActionOnReject")]
	pub should_launch_action_on_reject: bool,
}
