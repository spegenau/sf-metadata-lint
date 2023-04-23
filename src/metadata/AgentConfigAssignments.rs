use crate::metadata::AgentConfigProfileAssignments::AgentConfigProfileAssignments;
use crate::metadata::AgentConfigUserAssignments::AgentConfigUserAssignments;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AgentConfigAssignments  {
	#[serde(rename = "profiles")]
	pub profiles: Option<AgentConfigProfileAssignments>,
	#[serde(rename = "users")]
	pub users: Option<AgentConfigUserAssignments>,
}
