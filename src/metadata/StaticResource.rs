use crate::metadata::StaticResourceCacheControl::StaticResourceCacheControl;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StaticResource  {
	#[serde(rename = "cacheControl")]
	pub cache_control: StaticResourceCacheControl,
	#[serde(rename = "contentType")]
	pub content_type: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
}
