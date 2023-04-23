use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PresenceConfigProfileAssignments  {
	#[serde(rename = "profile")]
	pub profile: Option<Vec<String>>,
}
