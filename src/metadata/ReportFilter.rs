use crate::metadata::Language::Language;
use crate::metadata::ReportFilterItem::ReportFilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportFilter  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<ReportFilterItem>>,
	#[serde(rename = "language")]
	pub language: Option<Language>,
}
