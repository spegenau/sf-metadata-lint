use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardComponentGroupingSort  {
	#[serde(rename = "groupingLevel")]
	pub grouping_level: String,
	#[serde(rename = "inheritedReportGroupingSort")]
	pub inherited_report_grouping_sort: Option<String>,
	#[serde(rename = "sortColumn")]
	pub sort_column: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<String>,
}
