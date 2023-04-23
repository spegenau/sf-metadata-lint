use crate::metadata::CareRequestRecords::CareRequestRecords;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CareRequestConfiguration  {
	#[serde(rename = "careRequestRecordType")]
	pub care_request_record_type: String,
	#[serde(rename = "careRequestRecords")]
	pub care_request_records: Option<Vec<CareRequestRecords>>,
	#[serde(rename = "careRequestType")]
	pub care_request_type: String,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isDefaultRecordType")]
	pub is_default_record_type: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
