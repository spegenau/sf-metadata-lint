use crate::metadata::ReportChartComponentSize::ReportChartComponentSize;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportChartComponentLayoutItem  {
	#[serde(rename = "cacheData")]
	pub cache_data: Option<bool>,
	#[serde(rename = "contextFilterableField")]
	pub context_filterable_field: Option<String>,
	#[serde(rename = "error")]
	pub error: Option<String>,
	#[serde(rename = "hideOnError")]
	pub hide_on_error: Option<bool>,
	#[serde(rename = "includeContext")]
	pub include_context: Option<bool>,
	#[serde(rename = "reportName")]
	pub report_name: String,
	#[serde(rename = "showTitle")]
	pub show_title: Option<bool>,
	#[serde(rename = "size")]
	pub size: Option<ReportChartComponentSize>,
}
