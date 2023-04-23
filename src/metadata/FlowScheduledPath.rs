use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowScheduledPathOffsetUnit::FlowScheduledPathOffsetUnit;
use crate::metadata::FlowScheduledPathTimeSource::FlowScheduledPathTimeSource;
use crate::metadata::FlowScheduledPathType::FlowScheduledPathType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScheduledPath  {
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "maxBatchSize")]
	pub max_batch_size: Option<i32>,
	#[serde(rename = "offsetNumber")]
	pub offset_number: Option<i32>,
	#[serde(rename = "offsetUnit")]
	pub offset_unit: Option<FlowScheduledPathOffsetUnit>,
	#[serde(rename = "pathType")]
	pub path_type: Option<FlowScheduledPathType>,
	#[serde(rename = "recordField")]
	pub record_field: Option<String>,
	#[serde(rename = "timeSource")]
	pub time_source: Option<FlowScheduledPathTimeSource>,
}
