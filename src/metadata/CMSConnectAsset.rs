use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CMSConnectAsset  {
	#[serde(rename = "assetPath")]
	pub asset_path: String,
	#[serde(rename = "assetType")]
	pub asset_type: String,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
}
