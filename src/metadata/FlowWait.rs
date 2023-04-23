use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowWaitEvent::FlowWaitEvent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowWait  {
	#[serde(rename = "defaultConnector")]
	pub default_connector: Option<FlowConnector>,
	#[serde(rename = "defaultConnectorLabel")]
	pub default_connector_label: String,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "timeZoneId")]
	pub time_zone_id: Option<String>,
	#[serde(rename = "waitEvents")]
	pub wait_events: Option<Vec<FlowWaitEvent>>,
}
