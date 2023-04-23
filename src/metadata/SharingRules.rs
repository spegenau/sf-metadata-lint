use crate::metadata::SharingCriteriaRule::SharingCriteriaRule;
use crate::metadata::SharingGuestRule::SharingGuestRule;
use crate::metadata::SharingOwnerRule::SharingOwnerRule;
use crate::metadata::SharingTerritoryRule::SharingTerritoryRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingRules  {
	#[serde(rename = "sharingCriteriaRules")]
	pub sharing_criteria_rules: Option<Vec<SharingCriteriaRule>>,
	#[serde(rename = "sharingGuestRules")]
	pub sharing_guest_rules: Option<Vec<SharingGuestRule>>,
	#[serde(rename = "sharingOwnerRules")]
	pub sharing_owner_rules: Option<Vec<SharingOwnerRule>>,
	#[serde(rename = "sharingTerritoryRules")]
	pub sharing_territory_rules: Option<Vec<SharingTerritoryRule>>,
}
