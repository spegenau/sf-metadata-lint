use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceChannelStatus  {
	#[serde(rename = "channel")]
	pub channel: Option<Vec<String>>,
}
