use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveRecipe  {
	#[serde(rename = "application")]
	pub application: Option<String>,
	#[serde(rename = "dataflow")]
	pub dataflow: String,
	#[serde(rename = "format")]
	pub format: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "securityPredicate")]
	pub security_predicate: Option<String>,
	#[serde(rename = "targetDatasetAlias")]
	pub target_dataset_alias: Option<String>,
	#[serde(rename = "templateAssetSourceName")]
	pub template_asset_source_name: Option<String>,
}
