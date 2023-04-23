use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceChannelFieldPriority  {
	#[serde(rename = "priority")]
	pub priority: i32,
	#[serde(rename = "value")]
	pub value: String,
}
