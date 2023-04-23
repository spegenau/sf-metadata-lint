use crate::metadata::LicenseExpirationPolicy::LicenseExpirationPolicy;
use crate::metadata::PermissionSetLicenseDefinitionCustomPermission::PermissionSetLicenseDefinitionCustomPermission;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetLicenseDefinition  {
	#[serde(rename = "customPermissions")]
	pub custom_permissions: Option<Vec<PermissionSetLicenseDefinitionCustomPermission>>,
	#[serde(rename = "isSupplementLicense")]
	pub is_supplement_license: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "licenseExpirationPolicy")]
	pub license_expiration_policy: LicenseExpirationPolicy,
	#[serde(rename = "userLicenseRestrictions")]
	pub user_license_restrictions: Option<String>,
}
