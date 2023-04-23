use crate::metadata::AIApplicationStatus::AIApplicationStatus;
use crate::metadata::AIApplicationType::AIApplicationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AIApplication  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "status")]
	pub status: AIApplicationStatus,
	#[serde(rename = "type")]
	pub _type: AIApplicationType,
}
