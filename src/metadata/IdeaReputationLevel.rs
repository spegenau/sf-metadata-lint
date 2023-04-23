use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdeaReputationLevel  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: i32,
}
