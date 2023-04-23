use crate::metadata::DeployMessage::DeployMessage;
use crate::metadata::RetrieveResult::RetrieveResult;
use crate::metadata::RunTestsResult::RunTestsResult;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeployDetails  {
	#[serde(rename = "componentFailures")]
	pub component_failures: Option<Vec<DeployMessage>>,
	#[serde(rename = "componentSuccesses")]
	pub component_successes: Option<Vec<DeployMessage>>,
	#[serde(rename = "retrieveResult")]
	pub retrieve_result: Option<RetrieveResult>,
	#[serde(rename = "runTestResult")]
	pub run_test_result: Option<RunTestsResult>,
}
