use crate::metadata::IdentityVerificationDataSourceType::IdentityVerificationDataSourceType;
use crate::metadata::IdentityVerificationProcFld::IdentityVerificationProcFld;
use crate::metadata::IdentityVerificationSearchType::IdentityVerificationSearchType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdentityVerificationProcDtl  {
	#[serde(rename = "apexClass")]
	pub apex_class: Option<String>,
	#[serde(rename = "dataSourceType")]
	pub data_source_type: IdentityVerificationDataSourceType,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "identityVerificationProcFlds")]
	pub identity_verification_proc_flds: Option<Vec<IdentityVerificationProcFld>>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isRetryAllowedAfterLimit")]
	pub is_retry_allowed_after_limit: Option<bool>,
	#[serde(rename = "linkedIdVerfProcessDet")]
	pub linked_id_verf_process_det: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "objectName")]
	pub object_name: Option<String>,
	#[serde(rename = "optionalVerifiersMinVerfCount")]
	pub optional_verifiers_min_verf_count: Option<i32>,
	#[serde(rename = "retryLimit")]
	pub retry_limit: Option<i32>,
	#[serde(rename = "searchFilter")]
	pub search_filter: Option<String>,
	#[serde(rename = "searchRecordUniqueIdField")]
	pub search_record_unique_id_field: Option<String>,
	#[serde(rename = "searchResultSortBy")]
	pub search_result_sort_by: Option<String>,
	#[serde(rename = "searchSequenceNumber")]
	pub search_sequence_number: i32,
	#[serde(rename = "searchType")]
	pub search_type: IdentityVerificationSearchType,
}
