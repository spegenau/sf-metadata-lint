use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AgentConfigProfileAssignments  {
	#[serde(rename = "profile")]
	pub profile: Option<Vec<String>>,
}
