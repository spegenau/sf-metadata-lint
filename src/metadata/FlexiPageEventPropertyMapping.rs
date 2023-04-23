use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageEventPropertyMapping  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
