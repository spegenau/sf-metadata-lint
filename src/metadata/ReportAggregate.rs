use crate::metadata::ReportAggregateDatatype::ReportAggregateDatatype;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportAggregate  {
	#[serde(rename = "acrossGroupingContext")]
	pub across_grouping_context: Option<String>,
	#[serde(rename = "calculatedFormula")]
	pub calculated_formula: String,
	#[serde(rename = "datatype")]
	pub datatype: ReportAggregateDatatype,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "downGroupingContext")]
	pub down_grouping_context: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "isCrossBlock")]
	pub is_cross_block: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "reportType")]
	pub report_type: Option<String>,
	#[serde(rename = "scale")]
	pub scale: Option<i32>,
}
