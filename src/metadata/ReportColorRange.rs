use crate::metadata::ReportSummaryType::ReportSummaryType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportColorRange  {
	#[serde(rename = "aggregate")]
	pub aggregate: Option<ReportSummaryType>,
	#[serde(rename = "columnName")]
	pub column_name: String,
	#[serde(rename = "highBreakpoint")]
	pub high_breakpoint: Option<f32>,
	#[serde(rename = "highColor")]
	pub high_color: String,
	#[serde(rename = "lowBreakpoint")]
	pub low_breakpoint: Option<f32>,
	#[serde(rename = "lowColor")]
	pub low_color: String,
	#[serde(rename = "midColor")]
	pub mid_color: String,
}
