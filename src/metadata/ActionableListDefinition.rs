use crate::metadata::ActionableListDatasetColumn::ActionableListDatasetColumn;
use crate::metadata::ActionableListMemberStatus::ActionableListMemberStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionableListDefinition  {
	#[serde(rename = "actionableListDatasetColumns")]
	pub actionable_list_dataset_columns: Option<Vec<ActionableListDatasetColumn>>,
	#[serde(rename = "actionableListMemberStatuses")]
	pub actionable_list_member_statuses: Option<Vec<ActionableListMemberStatus>>,
	#[serde(rename = "batchCalcJobDefinition")]
	pub batch_calc_job_definition: Option<String>,
	#[serde(rename = "datasetName")]
	pub dataset_name: Option<String>,
	#[serde(rename = "edgeMart")]
	pub edge_mart: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "objectName")]
	pub object_name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
