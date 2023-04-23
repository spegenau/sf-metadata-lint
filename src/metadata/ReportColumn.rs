use crate::metadata::ReportSummaryType::ReportSummaryType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportColumn  {
	#[serde(rename = "aggregateTypes")]
	pub aggregate_types: Option<Vec<ReportSummaryType>>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "reverseColors")]
	pub reverse_colors: Option<bool>,
	#[serde(rename = "showChanges")]
	pub show_changes: Option<bool>,
}
