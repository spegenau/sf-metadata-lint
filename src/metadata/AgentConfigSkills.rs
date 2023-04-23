use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AgentConfigSkills  {
	#[serde(rename = "skill")]
	pub skill: Option<Vec<String>>,
}
