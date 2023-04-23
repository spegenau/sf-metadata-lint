use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::IterationOrder::IterationOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowLoop  {
	#[serde(rename = "assignNextValueToReference")]
	pub assign_next_value_to_reference: Option<String>,
	#[serde(rename = "collectionReference")]
	pub collection_reference: String,
	#[serde(rename = "iterationOrder")]
	pub iteration_order: Option<IterationOrder>,
	#[serde(rename = "nextValueConnector")]
	pub next_value_connector: Option<FlowConnector>,
	#[serde(rename = "noMoreValuesConnector")]
	pub no_more_values_connector: Option<FlowConnector>,
}
