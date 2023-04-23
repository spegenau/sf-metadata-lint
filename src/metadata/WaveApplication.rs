use crate::metadata::FolderShare::FolderShare;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveApplication  {
	#[serde(rename = "assetIcon")]
	pub asset_icon: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "folder")]
	pub folder: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "shares")]
	pub shares: Option<Vec<FolderShare>>,
	#[serde(rename = "templateOrigin")]
	pub template_origin: Option<String>,
	#[serde(rename = "templateVersion")]
	pub template_version: Option<String>,
}
