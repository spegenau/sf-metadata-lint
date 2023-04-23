use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowOutputFieldAssignment::FlowOutputFieldAssignment;
use crate::metadata::FlowRecordFilter::FlowRecordFilter;
use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowRecordLookup  {
	#[serde(rename = "assignNullValuesIfNoRecordsFound")]
	pub assign_null_values_if_no_records_found: Option<bool>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "filters")]
	pub filters: Option<Vec<FlowRecordFilter>>,
	#[serde(rename = "getFirstRecordOnly")]
	pub get_first_record_only: Option<bool>,
	#[serde(rename = "object")]
	pub object: String,
	#[serde(rename = "outputAssignments")]
	pub output_assignments: Option<Vec<FlowOutputFieldAssignment>>,
	#[serde(rename = "outputReference")]
	pub output_reference: Option<String>,
	#[serde(rename = "queriedFields")]
	pub queried_fields: Option<Vec<String>>,
	#[serde(rename = "sortField")]
	pub sort_field: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<SortOrder>,
	#[serde(rename = "storeOutputAutomatically")]
	pub store_output_automatically: Option<bool>,
}
