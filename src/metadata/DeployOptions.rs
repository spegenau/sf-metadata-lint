use crate::metadata::TestLevel::TestLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeployOptions  {
	#[serde(rename = "allowMissingFiles")]
	pub allow_missing_files: bool,
	#[serde(rename = "autoUpdatePackage")]
	pub auto_update_package: bool,
	#[serde(rename = "checkOnly")]
	pub check_only: bool,
	#[serde(rename = "ignoreWarnings")]
	pub ignore_warnings: bool,
	#[serde(rename = "performRetrieve")]
	pub perform_retrieve: bool,
	#[serde(rename = "purgeOnDelete")]
	pub purge_on_delete: bool,
	#[serde(rename = "rollbackOnError")]
	pub rollback_on_error: bool,
	#[serde(rename = "runTests")]
	pub run_tests: Option<Vec<String>>,
	#[serde(rename = "singlePackage")]
	pub single_package: bool,
	#[serde(rename = "testLevel")]
	pub test_level: TestLevel,
}
