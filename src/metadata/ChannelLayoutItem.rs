use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChannelLayoutItem  {
	#[serde(rename = "field")]
	pub field: String,
}
