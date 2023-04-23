use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageEventSourceProperty  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: String,
}
