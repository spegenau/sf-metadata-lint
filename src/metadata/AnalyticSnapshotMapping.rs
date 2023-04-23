use crate::metadata::ReportJobSourceTypes::ReportJobSourceTypes;
use crate::metadata::ReportSummaryType::ReportSummaryType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AnalyticSnapshotMapping  {
	#[serde(rename = "aggregateType")]
	pub aggregate_type: Option<ReportSummaryType>,
	#[serde(rename = "sourceField")]
	pub source_field: String,
	#[serde(rename = "sourceType")]
	pub source_type: ReportJobSourceTypes,
	#[serde(rename = "targetField")]
	pub target_field: String,
}
