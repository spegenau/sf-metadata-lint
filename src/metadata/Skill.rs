use crate::metadata::SkillAssignments::SkillAssignments;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Skill  {
	#[serde(rename = "assignments")]
	pub assignments: Option<SkillAssignments>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
