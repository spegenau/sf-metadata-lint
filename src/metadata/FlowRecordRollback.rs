use crate::metadata::FlowConnector::FlowConnector;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowRecordRollback  {
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
}
