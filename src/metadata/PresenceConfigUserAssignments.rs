use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PresenceConfigUserAssignments  {
	#[serde(rename = "user")]
	pub user: Option<Vec<String>>,
}
