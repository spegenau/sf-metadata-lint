use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordAlertCategory  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "severity")]
	pub severity: Option<String>,
}
