use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowStage  {
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "stageOrder")]
	pub stage_order: i32,
}
