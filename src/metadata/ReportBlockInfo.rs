use crate::metadata::ReportAggregateReference::ReportAggregateReference;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportBlockInfo  {
	#[serde(rename = "aggregateReferences")]
	pub aggregate_references: Option<Vec<ReportAggregateReference>>,
	#[serde(rename = "blockId")]
	pub block_id: String,
	#[serde(rename = "joinTable")]
	pub join_table: String,
}
