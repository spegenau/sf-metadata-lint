use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniSupervisorConfigSkill  {
	#[serde(rename = "skill")]
	pub skill: String,
}
