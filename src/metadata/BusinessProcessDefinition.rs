use crate::metadata::BusinessProcessFeedback::BusinessProcessFeedback;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BusinessProcessDefinition  {
	#[serde(rename = "businessProcessFeedbacks")]
	pub business_process_feedbacks: Option<Vec<BusinessProcessFeedback>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sequenceNumber")]
	pub sequence_number: i32,
}
