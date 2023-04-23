use crate::metadata::FlowCollectionMapItem::FlowCollectionMapItem;
use crate::metadata::FlowCollectionProcessorType::FlowCollectionProcessorType;
use crate::metadata::FlowCollectionSortOption::FlowCollectionSortOption;
use crate::metadata::FlowCondition::FlowCondition;
use crate::metadata::FlowConnector::FlowConnector;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCollectionProcessor  {
	#[serde(rename = "assignNextValueToReference")]
	pub assign_next_value_to_reference: Option<String>,
	#[serde(rename = "collectionProcessorType")]
	pub collection_processor_type: FlowCollectionProcessorType,
	#[serde(rename = "collectionReference")]
	pub collection_reference: String,
	#[serde(rename = "conditionLogic")]
	pub condition_logic: Option<String>,
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<FlowCondition>>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "limit")]
	pub limit: Option<i32>,
	#[serde(rename = "mapItems")]
	pub map_items: Option<Vec<FlowCollectionMapItem>>,
	#[serde(rename = "outputSObjectType")]
	pub output_s_object_type: Option<String>,
	#[serde(rename = "sortOptions")]
	pub sort_options: Option<Vec<FlowCollectionSortOption>>,
}
