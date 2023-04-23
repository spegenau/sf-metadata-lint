use crate::metadata::ReportFormattingRuleValue::ReportFormattingRuleValue;
use crate::metadata::ReportSummaryType::ReportSummaryType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportFormattingRule  {
	#[serde(rename = "aggregate")]
	pub aggregate: Option<ReportSummaryType>,
	#[serde(rename = "columnName")]
	pub column_name: String,
	#[serde(rename = "values")]
	pub values: Option<Vec<ReportFormattingRuleValue>>,
}
