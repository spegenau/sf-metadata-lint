use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Roles  {
	#[serde(rename = "role")]
	pub role: Option<Vec<String>>,
}
