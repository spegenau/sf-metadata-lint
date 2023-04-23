use crate::metadata::FilterOperation::FilterOperation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingFilterCondition  {
	#[serde(rename = "colName")]
	pub col_name: Option<String>,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "forecastingFilter")]
	pub forecasting_filter: String,
	#[serde(rename = "forecastingSourceDefinition")]
	pub forecasting_source_definition: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "operation")]
	pub operation: FilterOperation,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
