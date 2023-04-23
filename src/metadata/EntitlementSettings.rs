use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EntitlementSettings  {
	#[serde(rename = "assetLookupLimitedToActiveEntitlementsOnAccount")]
	pub asset_lookup_limited_to_active_entitlements_on_account: Option<bool>,
	#[serde(rename = "assetLookupLimitedToActiveEntitlementsOnContact")]
	pub asset_lookup_limited_to_active_entitlements_on_contact: Option<bool>,
	#[serde(rename = "assetLookupLimitedToSameAccount")]
	pub asset_lookup_limited_to_same_account: Option<bool>,
	#[serde(rename = "assetLookupLimitedToSameContact")]
	pub asset_lookup_limited_to_same_contact: Option<bool>,
	#[serde(rename = "enableEntitlementVersioning")]
	pub enable_entitlement_versioning: bool,
	#[serde(rename = "enableEntitlements")]
	pub enable_entitlements: bool,
	#[serde(rename = "enableMilestoneFeedItem")]
	pub enable_milestone_feed_item: Option<bool>,
	#[serde(rename = "enableMilestoneStoppedTime")]
	pub enable_milestone_stopped_time: Option<bool>,
	#[serde(rename = "entitlementLookupLimitedToActiveStatus")]
	pub entitlement_lookup_limited_to_active_status: Option<bool>,
	#[serde(rename = "entitlementLookupLimitedToSameAccount")]
	pub entitlement_lookup_limited_to_same_account: Option<bool>,
	#[serde(rename = "entitlementLookupLimitedToSameAsset")]
	pub entitlement_lookup_limited_to_same_asset: Option<bool>,
	#[serde(rename = "entitlementLookupLimitedToSameContact")]
	pub entitlement_lookup_limited_to_same_contact: Option<bool>,
	#[serde(rename = "ignoreMilestoneBusinessHours")]
	pub ignore_milestone_business_hours: Option<bool>,
}
