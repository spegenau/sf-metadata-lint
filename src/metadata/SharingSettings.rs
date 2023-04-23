use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingSettings  {
	#[serde(rename = "deferGroupMembership")]
	pub defer_group_membership: Option<bool>,
	#[serde(rename = "deferSharingRules")]
	pub defer_sharing_rules: Option<bool>,
	#[serde(rename = "enableAccountRoleOptimization")]
	pub enable_account_role_optimization: Option<bool>,
	#[serde(rename = "enableAssetSharing")]
	pub enable_asset_sharing: Option<bool>,
	#[serde(rename = "enableCommunityUserVisibility")]
	pub enable_community_user_visibility: Option<bool>,
	#[serde(rename = "enableExternalSharingModel")]
	pub enable_external_sharing_model: Option<bool>,
	#[serde(rename = "enableManagerGroups")]
	pub enable_manager_groups: Option<bool>,
	#[serde(rename = "enableManualUserRecordSharing")]
	pub enable_manual_user_record_sharing: Option<bool>,
	#[serde(rename = "enablePartnerSuperUserAccess")]
	pub enable_partner_super_user_access: Option<bool>,
	#[serde(rename = "enablePortalUserCaseSharing")]
	pub enable_portal_user_case_sharing: Option<bool>,
	#[serde(rename = "enablePortalUserVisibility")]
	pub enable_portal_user_visibility: Option<bool>,
	#[serde(rename = "enableRemoveTMGroupMembership")]
	pub enable_remove_tm_group_membership: Option<bool>,
	#[serde(rename = "enableRestrictAccessLookupRecords")]
	pub enable_restrict_access_lookup_records: Option<bool>,
	#[serde(rename = "enableSecureGuestAccess")]
	pub enable_secure_guest_access: Option<bool>,
	#[serde(rename = "enableShareObjectReportTypes")]
	pub enable_share_object_report_types: Option<bool>,
	#[serde(rename = "enableStandardReportVisibility")]
	pub enable_standard_report_visibility: Option<bool>,
	#[serde(rename = "enableTerritoryForecastManager")]
	pub enable_territory_forecast_manager: Option<bool>,
}
