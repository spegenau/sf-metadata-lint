use crate::metadata::DeployDetails::DeployDetails;
use crate::metadata::DeployStatus::DeployStatus;
use crate::metadata::ID::ID;
use crate::metadata::StatusCode::StatusCode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeployResult  {
	#[serde(rename = "canceledBy")]
	pub canceled_by: Option<String>,
	#[serde(rename = "canceledByName")]
	pub canceled_by_name: Option<String>,
	#[serde(rename = "checkOnly")]
	pub check_only: bool,
	#[serde(rename = "completedDate")]
	pub completed_date: Option<String>,
	#[serde(rename = "createdBy")]
	pub created_by: String,
	#[serde(rename = "createdByName")]
	pub created_by_name: String,
	#[serde(rename = "createdDate")]
	pub created_date: String,
	#[serde(rename = "details")]
	pub details: DeployDetails,
	#[serde(rename = "done")]
	pub done: bool,
	#[serde(rename = "errorMessage")]
	pub error_message: Option<String>,
	#[serde(rename = "errorStatusCode")]
	pub error_status_code: Option<StatusCode>,
	#[serde(rename = "id")]
	pub id: ID,
	#[serde(rename = "ignoreWarnings")]
	pub ignore_warnings: bool,
	#[serde(rename = "lastModifiedDate")]
	pub last_modified_date: Option<String>,
	#[serde(rename = "numberComponentErrors")]
	pub number_component_errors: i32,
	#[serde(rename = "numberComponentsDeployed")]
	pub number_components_deployed: i32,
	#[serde(rename = "numberComponentsTotal")]
	pub number_components_total: i32,
	#[serde(rename = "numberTestErrors")]
	pub number_test_errors: i32,
	#[serde(rename = "numberTestsCompleted")]
	pub number_tests_completed: i32,
	#[serde(rename = "numberTestsTotal")]
	pub number_tests_total: i32,
	#[serde(rename = "rollbackOnError")]
	pub rollback_on_error: bool,
	#[serde(rename = "runTestsEnabled")]
	pub run_tests_enabled: bool,
	#[serde(rename = "startDate")]
	pub start_date: Option<String>,
	#[serde(rename = "stateDetail")]
	pub state_detail: Option<String>,
	#[serde(rename = "status")]
	pub status: DeployStatus,
	#[serde(rename = "success")]
	pub success: bool,
}
