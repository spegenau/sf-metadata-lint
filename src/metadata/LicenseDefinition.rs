use crate::metadata::LicensedCustomPermissions::LicensedCustomPermissions;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LicenseDefinition  {
	#[serde(rename = "aggregationGroup")]
	pub aggregation_group: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isPublished")]
	pub is_published: bool,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "licensedCustomPermissions")]
	pub licensed_custom_permissions: Option<Vec<LicensedCustomPermissions>>,
	#[serde(rename = "licensingAuthority")]
	pub licensing_authority: String,
	#[serde(rename = "licensingAuthorityProvider")]
	pub licensing_authority_provider: String,
	#[serde(rename = "minPlatformVersion")]
	pub min_platform_version: i32,
	#[serde(rename = "origin")]
	pub origin: String,
	#[serde(rename = "revision")]
	pub revision: i32,
	#[serde(rename = "trialLicenseDuration")]
	pub trial_license_duration: i32,
	#[serde(rename = "trialLicenseQuantity")]
	pub trial_license_quantity: i32,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
