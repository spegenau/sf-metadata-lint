use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldInstanceProperty  {
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
