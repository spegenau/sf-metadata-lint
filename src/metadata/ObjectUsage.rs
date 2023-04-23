use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectUsage  {
	#[serde(rename = "object")]
	pub object: Option<Vec<String>>,
}
