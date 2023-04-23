use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AnalyticsCloudComponentLayoutItem  {
	#[serde(rename = "assetType")]
	pub asset_type: String,
	#[serde(rename = "devName")]
	pub dev_name: String,
	#[serde(rename = "error")]
	pub error: Option<String>,
	#[serde(rename = "filter")]
	pub filter: Option<String>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "hideOnError")]
	pub hide_on_error: Option<bool>,
	#[serde(rename = "showHeader")]
	pub show_header: Option<bool>,
	#[serde(rename = "showSharing")]
	pub show_sharing: Option<bool>,
	#[serde(rename = "showTitle")]
	pub show_title: Option<bool>,
	#[serde(rename = "width")]
	pub width: Option<String>,
}
