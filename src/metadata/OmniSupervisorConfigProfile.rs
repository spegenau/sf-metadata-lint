use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniSupervisorConfigProfile  {
	#[serde(rename = "profile")]
	pub profile: String,
}
