use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Metadata  {
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
