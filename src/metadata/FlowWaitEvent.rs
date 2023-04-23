use crate::metadata::FlowCondition::FlowCondition;
use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowScheduledPathOffsetUnit::FlowScheduledPathOffsetUnit;
use crate::metadata::FlowWaitEventInputParameter::FlowWaitEventInputParameter;
use crate::metadata::FlowWaitEventOutputParameter::FlowWaitEventOutputParameter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowWaitEvent  {
	#[serde(rename = "conditionLogic")]
	pub condition_logic: Option<String>,
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<FlowCondition>>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "eventType")]
	pub event_type: Option<String>,
	#[serde(rename = "inputParameters")]
	pub input_parameters: Option<Vec<FlowWaitEventInputParameter>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "maxBatchSize")]
	pub max_batch_size: Option<i32>,
	#[serde(rename = "offset")]
	pub offset: Option<i32>,
	#[serde(rename = "offsetUnit")]
	pub offset_unit: Option<FlowScheduledPathOffsetUnit>,
	#[serde(rename = "outputParameters")]
	pub output_parameters: Option<Vec<FlowWaitEventOutputParameter>>,
	#[serde(rename = "resumeDate")]
	pub resume_date: Option<String>,
	#[serde(rename = "resumeDateReference")]
	pub resume_date_reference: Option<String>,
	#[serde(rename = "resumeTime")]
	pub resume_time: Option<String>,
}
