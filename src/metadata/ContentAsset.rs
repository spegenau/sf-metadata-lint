use crate::metadata::ContentAssetFormat::ContentAssetFormat;
use crate::metadata::ContentAssetRelationships::ContentAssetRelationships;
use crate::metadata::ContentAssetVersions::ContentAssetVersions;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContentAsset  {
	#[serde(rename = "format")]
	pub format: Option<ContentAssetFormat>,
	#[serde(rename = "isVisibleByExternalUsers")]
	pub is_visible_by_external_users: Option<bool>,
	#[serde(rename = "language")]
	pub language: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "originNetwork")]
	pub origin_network: Option<String>,
	#[serde(rename = "relationships")]
	pub relationships: Option<ContentAssetRelationships>,
	#[serde(rename = "versions")]
	pub versions: ContentAssetVersions,
}
