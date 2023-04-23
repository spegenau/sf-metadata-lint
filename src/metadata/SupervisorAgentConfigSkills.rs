use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SupervisorAgentConfigSkills  {
	#[serde(rename = "skill")]
	pub skill: Option<Vec<String>>,
}
