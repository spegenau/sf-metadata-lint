use crate::metadata::ProcessSubmitterType::ProcessSubmitterType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalSubmitter  {
	#[serde(rename = "submitter")]
	pub submitter: Option<String>,
	#[serde(rename = "type")]
	pub _type: ProcessSubmitterType,
}
