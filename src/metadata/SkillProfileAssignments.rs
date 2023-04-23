use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SkillProfileAssignments  {
	#[serde(rename = "profile")]
	pub profile: Option<Vec<String>>,
}
