use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CMSConnectPersonalization  {
	#[serde(rename = "connectorPage")]
	pub connector_page: String,
	#[serde(rename = "connectorPageAsset")]
	pub connector_page_asset: String,
}
