use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::FlowOutputFieldAssignment::FlowOutputFieldAssignment;
use crate::metadata::FlowRecordFilter::FlowRecordFilter;
use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowDynamicChoiceSet  {
	#[serde(rename = "collectionReference")]
	pub collection_reference: Option<String>,
	#[serde(rename = "dataType")]
	pub data_type: FlowDataType,
	#[serde(rename = "displayField")]
	pub display_field: String,
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "filters")]
	pub filters: Option<Vec<FlowRecordFilter>>,
	#[serde(rename = "limit")]
	pub limit: Option<i32>,
	#[serde(rename = "object")]
	pub object: String,
	#[serde(rename = "outputAssignments")]
	pub output_assignments: Option<Vec<FlowOutputFieldAssignment>>,
	#[serde(rename = "picklistField")]
	pub picklist_field: Option<String>,
	#[serde(rename = "picklistObject")]
	pub picklist_object: Option<String>,
	#[serde(rename = "sortField")]
	pub sort_field: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<SortOrder>,
	#[serde(rename = "valueField")]
	pub value_field: Option<String>,
}
