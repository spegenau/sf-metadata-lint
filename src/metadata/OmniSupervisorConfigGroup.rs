use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniSupervisorConfigGroup  {
	#[serde(rename = "group")]
	pub group: String,
}
