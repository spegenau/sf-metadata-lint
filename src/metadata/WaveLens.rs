use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveLens  {
	#[serde(rename = "application")]
	pub application: String,
	#[serde(rename = "datasets")]
	pub datasets: Option<Vec<String>>,
	#[serde(rename = "dateVersion")]
	pub date_version: Option<i32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "templateAssetSourceName")]
	pub template_asset_source_name: Option<String>,
	#[serde(rename = "visualizationType")]
	pub visualization_type: String,
}
