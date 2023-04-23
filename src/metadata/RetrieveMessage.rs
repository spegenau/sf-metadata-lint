use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RetrieveMessage  {
	#[serde(rename = "fileName")]
	pub file_name: String,
	#[serde(rename = "problem")]
	pub problem: String,
}
