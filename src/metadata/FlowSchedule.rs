use crate::metadata::FlowStartFrequency::FlowStartFrequency;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowSchedule  {
	#[serde(rename = "frequency")]
	pub frequency: Option<FlowStartFrequency>,
	#[serde(rename = "startDate")]
	pub start_date: Option<String>,
	#[serde(rename = "startTime")]
	pub start_time: Option<String>,
}
