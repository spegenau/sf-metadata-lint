use crate::metadata::PermissionSetApexClassAccess::PermissionSetApexClassAccess;
use crate::metadata::PermissionSetApexPageAccess::PermissionSetApexPageAccess;
use crate::metadata::PermissionSetApplicationVisibility::PermissionSetApplicationVisibility;
use crate::metadata::PermissionSetCustomMetadataTypeAccess::PermissionSetCustomMetadataTypeAccess;
use crate::metadata::PermissionSetCustomPermissions::PermissionSetCustomPermissions;
use crate::metadata::PermissionSetCustomSettingAccess::PermissionSetCustomSettingAccess;
use crate::metadata::PermissionSetExternalDataSourceAccess::PermissionSetExternalDataSourceAccess;
use crate::metadata::PermissionSetFieldPermissions::PermissionSetFieldPermissions;
use crate::metadata::PermissionSetFlowAccess::PermissionSetFlowAccess;
use crate::metadata::PermissionSetObjectPermissions::PermissionSetObjectPermissions;
use crate::metadata::PermissionSetRecordTypeVisibility::PermissionSetRecordTypeVisibility;
use crate::metadata::PermissionSetTabSetting::PermissionSetTabSetting;
use crate::metadata::PermissionSetUserPermission::PermissionSetUserPermission;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSet  {
	#[serde(rename = "applicationVisibilities")]
	pub application_visibilities: Option<Vec<PermissionSetApplicationVisibility>>,
	#[serde(rename = "classAccesses")]
	pub class_accesses: Option<Vec<PermissionSetApexClassAccess>>,
	#[serde(rename = "customMetadataTypeAccesses")]
	pub custom_metadata_type_accesses: Option<Vec<PermissionSetCustomMetadataTypeAccess>>,
	#[serde(rename = "customPermissions")]
	pub custom_permissions: Option<Vec<PermissionSetCustomPermissions>>,
	#[serde(rename = "customSettingAccesses")]
	pub custom_setting_accesses: Option<Vec<PermissionSetCustomSettingAccess>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "externalDataSourceAccesses")]
	pub external_data_source_accesses: Option<Vec<PermissionSetExternalDataSourceAccess>>,
	#[serde(rename = "fieldPermissions")]
	pub field_permissions: Option<Vec<PermissionSetFieldPermissions>>,
	#[serde(rename = "flowAccesses")]
	pub flow_accesses: Option<Vec<PermissionSetFlowAccess>>,
	#[serde(rename = "hasActivationRequired")]
	pub has_activation_required: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "license")]
	pub license: Option<String>,
	#[serde(rename = "objectPermissions")]
	pub object_permissions: Option<Vec<PermissionSetObjectPermissions>>,
	#[serde(rename = "pageAccesses")]
	pub page_accesses: Option<Vec<PermissionSetApexPageAccess>>,
	#[serde(rename = "recordTypeVisibilities")]
	pub record_type_visibilities: Option<Vec<PermissionSetRecordTypeVisibility>>,
	#[serde(rename = "tabSettings")]
	pub tab_settings: Option<Vec<PermissionSetTabSetting>>,
	#[serde(rename = "userPermissions")]
	pub user_permissions: Option<Vec<PermissionSetUserPermission>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
