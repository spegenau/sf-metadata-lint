use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AgentConfigUserAssignments  {
	#[serde(rename = "user")]
	pub user: Option<Vec<String>>,
}
