use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlexiPageEventTargetProperty  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: String,
}
