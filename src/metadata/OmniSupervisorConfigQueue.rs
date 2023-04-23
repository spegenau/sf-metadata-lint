use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniSupervisorConfigQueue  {
	#[serde(rename = "queue")]
	pub queue: String,
}
