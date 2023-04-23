use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdDimensionMember  {
	#[serde(rename = "color")]
	pub color: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "member")]
	pub member: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
