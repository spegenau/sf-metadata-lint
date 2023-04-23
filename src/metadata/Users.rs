use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Users  {
	#[serde(rename = "user")]
	pub user: Option<Vec<String>>,
}
