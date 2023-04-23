use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PartyDataModelSettings  {
	#[serde(rename = "enableAutoSelectIndividualOnMerge")]
	pub enable_auto_select_individual_on_merge: Option<bool>,
	#[serde(rename = "enableConsentManagement")]
	pub enable_consent_management: Option<bool>,
	#[serde(rename = "enableIndividualAutoCreate")]
	pub enable_individual_auto_create: Option<bool>,
}
