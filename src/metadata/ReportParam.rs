use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportParam  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: String,
}
