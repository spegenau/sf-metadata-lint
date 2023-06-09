use crate::metadata::AssignmentRule::AssignmentRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AssignmentRules  {
	#[serde(rename = "assignmentRule")]
	pub assignment_rule: Option<Vec<AssignmentRule>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
