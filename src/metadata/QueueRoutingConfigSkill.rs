use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QueueRoutingConfigSkill  {
	#[serde(rename = "skill")]
	pub skill: Option<String>,
}
