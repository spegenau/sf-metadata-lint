use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveDataset  {
	#[serde(rename = "application")]
	pub application: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "templateAssetSourceName")]
	pub template_asset_source_name: Option<String>,
	#[serde(rename = "type")]
	pub _type: Option<String>,
}
