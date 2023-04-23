use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SkillUserAssignments  {
	#[serde(rename = "user")]
	pub user: Option<Vec<String>>,
}
