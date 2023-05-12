use crate::metadata::LoginFlow::LoginFlow;
use crate::metadata::ProfileActionOverride::ProfileActionOverride;
use crate::metadata::ProfileApexClassAccess::ProfileApexClassAccess;
use crate::metadata::ProfileApexPageAccess::ProfileApexPageAccess;
use crate::metadata::ProfileApplicationVisibility::ProfileApplicationVisibility;
use crate::metadata::ProfileCategoryGroupVisibility::ProfileCategoryGroupVisibility;
use crate::metadata::ProfileCustomMetadataTypeAccess::ProfileCustomMetadataTypeAccess;
use crate::metadata::ProfileCustomPermissions::ProfileCustomPermissions;
use crate::metadata::ProfileCustomSettingAccess::ProfileCustomSettingAccess;
use crate::metadata::ProfileExternalDataSourceAccess::ProfileExternalDataSourceAccess;
use crate::metadata::ProfileFieldLevelSecurity::ProfileFieldLevelSecurity;
use crate::metadata::ProfileFlowAccess::ProfileFlowAccess;
use crate::metadata::ProfileLayoutAssignment::ProfileLayoutAssignment;
use crate::metadata::ProfileLoginHours::ProfileLoginHours;
use crate::metadata::ProfileLoginIpRange::ProfileLoginIpRange;
use crate::metadata::ProfileObjectPermissions::ProfileObjectPermissions;
use crate::metadata::ProfileRecordTypeVisibility::ProfileRecordTypeVisibility;
use crate::metadata::ProfileTabVisibility::ProfileTabVisibility;
use crate::metadata::ProfileUserPermission::ProfileUserPermission;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Profile  {
	#[serde(rename = "applicationVisibilities")]
	pub application_visibilities: Option<Vec<ProfileApplicationVisibility>>,
	#[serde(rename = "categoryGroupVisibilities")]
	pub category_group_visibilities: Option<Vec<ProfileCategoryGroupVisibility>>,
	#[serde(rename = "classAccesses")]
	pub class_accesses: Option<Vec<ProfileApexClassAccess>>,
	#[serde(rename = "custom")]
	pub custom: Option<bool>,
	#[serde(rename = "customMetadataTypeAccesses")]
	pub custom_metadata_type_accesses: Option<Vec<ProfileCustomMetadataTypeAccess>>,
	#[serde(rename = "customPermissions")]
	pub custom_permissions: Option<Vec<ProfileCustomPermissions>>,
	#[serde(rename = "customSettingAccesses")]
	pub custom_setting_accesses: Option<Vec<ProfileCustomSettingAccess>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "externalDataSourceAccesses")]
	pub external_data_source_accesses: Option<Vec<ProfileExternalDataSourceAccess>>,
	#[serde(rename = "fieldPermissions")]
	pub field_permissions: Option<Vec<ProfileFieldLevelSecurity>>,
	#[serde(rename = "flowAccesses")]
	pub flow_accesses: Option<Vec<ProfileFlowAccess>>,
	#[serde(rename = "layoutAssignments")]
	pub layout_assignments: Option<Vec<ProfileLayoutAssignment>>,
	#[serde(rename = "loginFlows")]
	pub login_flows: Option<Vec<LoginFlow>>,
	#[serde(rename = "loginHours")]
	pub login_hours: Option<ProfileLoginHours>,
	#[serde(rename = "loginIpRanges")]
	pub login_ip_ranges: Option<Vec<ProfileLoginIpRange>>,
	#[serde(rename = "objectPermissions")]
	pub object_permissions: Option<Vec<ProfileObjectPermissions>>,
	#[serde(rename = "pageAccesses")]
	pub page_accesses: Option<Vec<ProfileApexPageAccess>>,
	#[serde(rename = "profileActionOverrides")]
	pub profile_action_overrides: Option<Vec<ProfileActionOverride>>,
	#[serde(rename = "recordTypeVisibilities")]
	pub record_type_visibilities: Option<Vec<ProfileRecordTypeVisibility>>,
	#[serde(rename = "tabVisibilities")]
	pub tab_visibilities: Option<Vec<ProfileTabVisibility>>,
	#[serde(rename = "userLicense")]
	pub user_license: Option<String>,
	#[serde(rename = "userPermissions")]
	pub user_permissions: Option<Vec<ProfileUserPermission>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
