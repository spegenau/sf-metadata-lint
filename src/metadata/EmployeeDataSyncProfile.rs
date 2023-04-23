use crate::metadata::EmployeeDataSyncField::EmployeeDataSyncField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmployeeDataSyncProfile  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "employeeDataSyncField")]
	pub employee_data_sync_field: Option<Vec<EmployeeDataSyncField>>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
