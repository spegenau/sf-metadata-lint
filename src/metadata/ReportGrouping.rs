use crate::metadata::ReportAggrType::ReportAggrType;
use crate::metadata::ReportSortType::ReportSortType;
use crate::metadata::SortOrder::SortOrder;
use crate::metadata::UserDateGranularity::UserDateGranularity;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportGrouping  {
	#[serde(rename = "aggregateType")]
	pub aggregate_type: Option<ReportAggrType>,
	#[serde(rename = "dateGranularity")]
	pub date_granularity: Option<UserDateGranularity>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "sortByName")]
	pub sort_by_name: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: SortOrder,
	#[serde(rename = "sortType")]
	pub sort_type: Option<ReportSortType>,
}
