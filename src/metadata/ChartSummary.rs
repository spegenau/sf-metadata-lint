use crate::metadata::ChartAxis::ChartAxis;
use crate::metadata::ReportSummaryType::ReportSummaryType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChartSummary  {
	#[serde(rename = "aggregate")]
	pub aggregate: Option<ReportSummaryType>,
	#[serde(rename = "axisBinding")]
	pub axis_binding: Option<ChartAxis>,
	#[serde(rename = "column")]
	pub column: String,
}
