use crate::metadata::PlatformCacheType::PlatformCacheType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformCachePartitionType  {
	#[serde(rename = "allocatedCapacity")]
	pub allocated_capacity: i32,
	#[serde(rename = "allocatedPartnerCapacity")]
	pub allocated_partner_capacity: i32,
	#[serde(rename = "allocatedPurchasedCapacity")]
	pub allocated_purchased_capacity: i32,
	#[serde(rename = "allocatedTrialCapacity")]
	pub allocated_trial_capacity: i32,
	#[serde(rename = "cacheType")]
	pub cache_type: PlatformCacheType,
}
