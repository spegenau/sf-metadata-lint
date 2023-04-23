use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContactCenterChannel  {
	#[serde(rename = "channel")]
	pub channel: String,
}
