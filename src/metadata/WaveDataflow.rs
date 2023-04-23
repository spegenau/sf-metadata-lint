use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveDataflow  {
	#[serde(rename = "application")]
	pub application: Option<String>,
	#[serde(rename = "dataflowType")]
	pub dataflow_type: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
