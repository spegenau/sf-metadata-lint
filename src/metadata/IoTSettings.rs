use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IoTSettings  {
	#[serde(rename = "enableIoT")]
	pub enable_io_t: Option<bool>,
	#[serde(rename = "enableIoTInsightsPilot")]
	pub enable_io_t_insights_pilot: Option<bool>,
	#[serde(rename = "enableIoTUsageEmail")]
	pub enable_io_t_usage_email: Option<bool>,
}
