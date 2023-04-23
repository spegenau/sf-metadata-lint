use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Capabilities  {
	#[serde(rename = "capability")]
	pub capability: Option<Vec<String>>,
}
