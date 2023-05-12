use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FilesConnectSettings  {
	#[serde(rename = "enableContentHubAllowed")]
	pub enable_content_hub_allowed: Option<bool>,
	#[serde(rename = "enableContentHubCvtLinksAllowed")]
	pub enable_content_hub_cvt_links_allowed: Option<bool>,
	#[serde(rename = "enableContentHubEOSearchLayout")]
	pub enable_content_hub_eo_search_layout: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
