use crate::metadata::DashboardComponentFilter::DashboardComponentFilter;
use crate::metadata::ReportSummaryType::ReportSummaryType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardTableColumn  {
	#[serde(rename = "aggregateType")]
	pub aggregate_type: Option<ReportSummaryType>,
	#[serde(rename = "calculatePercent")]
	pub calculate_percent: Option<bool>,
	#[serde(rename = "column")]
	pub column: String,
	#[serde(rename = "decimalPlaces")]
	pub decimal_places: Option<i32>,
	#[serde(rename = "showSubTotal")]
	pub show_sub_total: Option<bool>,
	#[serde(rename = "showTotal")]
	pub show_total: Option<bool>,
	#[serde(rename = "sortBy")]
	pub sort_by: Option<DashboardComponentFilter>,
}
