use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdDimensionCustomAction  {
	#[serde(rename = "customActionName")]
	pub custom_action_name: String,
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "method")]
	pub method: Option<String>,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
	#[serde(rename = "target")]
	pub target: Option<String>,
	#[serde(rename = "tooltip")]
	pub tooltip: Option<String>,
	#[serde(rename = "url")]
	pub url: Option<String>,
}
