use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReputationPointsRule  {
	#[serde(rename = "eventType")]
	pub event_type: String,
	#[serde(rename = "points")]
	pub points: i32,
}
