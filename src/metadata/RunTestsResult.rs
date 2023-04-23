use crate::metadata::CodeCoverageResult::CodeCoverageResult;
use crate::metadata::CodeCoverageWarning::CodeCoverageWarning;
use crate::metadata::FlowCoverageResult::FlowCoverageResult;
use crate::metadata::FlowCoverageWarning::FlowCoverageWarning;
use crate::metadata::RunTestFailure::RunTestFailure;
use crate::metadata::RunTestSuccess::RunTestSuccess;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RunTestsResult  {
	#[serde(rename = "apexLogId")]
	pub apex_log_id: Option<String>,
	#[serde(rename = "codeCoverage")]
	pub code_coverage: Option<Vec<CodeCoverageResult>>,
	#[serde(rename = "codeCoverageWarnings")]
	pub code_coverage_warnings: Option<Vec<CodeCoverageWarning>>,
	#[serde(rename = "failures")]
	pub failures: Option<Vec<RunTestFailure>>,
	#[serde(rename = "flowCoverage")]
	pub flow_coverage: Option<Vec<FlowCoverageResult>>,
	#[serde(rename = "flowCoverageWarnings")]
	pub flow_coverage_warnings: Option<Vec<FlowCoverageWarning>>,
	#[serde(rename = "numFailures")]
	pub num_failures: i32,
	#[serde(rename = "numTestsRun")]
	pub num_tests_run: i32,
	#[serde(rename = "successes")]
	pub successes: Option<Vec<RunTestSuccess>>,
	#[serde(rename = "totalTime")]
	pub total_time: f32,
}
