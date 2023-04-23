use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Attachment  {
	#[serde(rename = "content")]
	pub content: String,
	#[serde(rename = "name")]
	pub name: String,
}
