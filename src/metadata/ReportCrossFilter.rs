use crate::metadata::ObjectFilterOperator::ObjectFilterOperator;
use crate::metadata::ReportFilterItem::ReportFilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportCrossFilter  {
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<ReportFilterItem>>,
	#[serde(rename = "operation")]
	pub operation: ObjectFilterOperator,
	#[serde(rename = "primaryTableColumn")]
	pub primary_table_column: String,
	#[serde(rename = "relatedTable")]
	pub related_table: String,
	#[serde(rename = "relatedTableJoinColumn")]
	pub related_table_join_column: String,
}
