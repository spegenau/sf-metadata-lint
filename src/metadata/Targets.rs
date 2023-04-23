use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Targets  {
	#[serde(rename = "target")]
	pub target: Option<Vec<String>>,
}
