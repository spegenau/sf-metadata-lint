use crate::metadata::SkillProfileAssignments::SkillProfileAssignments;
use crate::metadata::SkillUserAssignments::SkillUserAssignments;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SkillAssignments  {
	#[serde(rename = "profiles")]
	pub profiles: Option<SkillProfileAssignments>,
	#[serde(rename = "users")]
	pub users: Option<SkillUserAssignments>,
}
