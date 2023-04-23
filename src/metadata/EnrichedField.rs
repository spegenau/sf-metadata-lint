use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EnrichedField  {
	#[serde(rename = "name")]
	pub name: String,
}
