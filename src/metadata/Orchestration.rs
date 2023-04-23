use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Orchestration  {
	#[serde(rename = "context")]
	pub context: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
