use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PersonalizationTargetInfo  {
	#[serde(rename = "groupName")]
	pub group_name: String,
	#[serde(rename = "priority")]
	pub priority: Option<i32>,
	#[serde(rename = "targetType")]
	pub target_type: String,
	#[serde(rename = "targetValue")]
	pub target_value: String,
}
