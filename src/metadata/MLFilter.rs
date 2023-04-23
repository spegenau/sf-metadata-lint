use crate::metadata::AIFilterOperation::AIFilterOperation;
use crate::metadata::AIFilterUnit::AIFilterUnit;
use crate::metadata::AIValueType::AIValueType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MLFilter  {
	#[serde(rename = "filterName")]
	pub filter_name: String,
	#[serde(rename = "lhFilter")]
	pub lh_filter: Option<Box<MLFilter>>,
	#[serde(rename = "lhPredictionField")]
	pub lh_prediction_field: Option<String>,
	#[serde(rename = "lhType")]
	pub lh_type: Option<AIValueType>,
	#[serde(rename = "lhUnit")]
	pub lh_unit: Option<AIFilterUnit>,
	#[serde(rename = "lhValue")]
	pub lh_value: Option<String>,
	#[serde(rename = "operation")]
	pub operation: AIFilterOperation,
	#[serde(rename = "rhFilter")]
	pub rh_filter: Option<Box<MLFilter>>,
	#[serde(rename = "rhPredictionField")]
	pub rh_prediction_field: Option<String>,
	#[serde(rename = "rhType")]
	pub rh_type: Option<AIValueType>,
	#[serde(rename = "rhUnit")]
	pub rh_unit: Option<AIFilterUnit>,
	#[serde(rename = "rhValue")]
	pub rh_value: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<i32>,
}
