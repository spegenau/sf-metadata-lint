use crate::metadata::PlatformCachePartitionType::PlatformCachePartitionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformCachePartition  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isDefaultPartition")]
	pub is_default_partition: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "platformCachePartitionTypes")]
	pub platform_cache_partition_types: Option<Vec<PlatformCachePartitionType>>,
}
