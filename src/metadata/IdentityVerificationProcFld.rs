use crate::metadata::IdentityVerificationProcFldDataSourceType::IdentityVerificationProcFldDataSourceType;
use crate::metadata::IdentityVerificationProcFldFieldDataType::IdentityVerificationProcFldFieldDataType;
use crate::metadata::IdentityVerificationProcFldFieldType::IdentityVerificationProcFldFieldType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdentityVerificationProcFld  {
	#[serde(rename = "customFieldLabel")]
	pub custom_field_label: Option<String>,
	#[serde(rename = "dataSourceType")]
	pub data_source_type: IdentityVerificationProcFldDataSourceType,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "fieldDataType")]
	pub field_data_type: Option<IdentityVerificationProcFldFieldDataType>,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "fieldType")]
	pub field_type: IdentityVerificationProcFldFieldType,
	#[serde(rename = "fieldValueFormula")]
	pub field_value_formula: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isManualInput")]
	pub is_manual_input: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sequenceNumber")]
	pub sequence_number: i32,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
