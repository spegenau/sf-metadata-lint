use crate::metadata::MlSlotClassDataType::MlSlotClassDataType;
use crate::metadata::MlSlotClassExtractionType::MlSlotClassExtractionType;
use crate::metadata::MlSlotClassValue::MlSlotClassValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MlSlotClass  {
	#[serde(rename = "dataType")]
	pub data_type: MlSlotClassDataType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "extractionRegex")]
	pub extraction_regex: Option<String>,
	#[serde(rename = "extractionType")]
	pub extraction_type: Option<MlSlotClassExtractionType>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mlSlotClassValues")]
	pub ml_slot_class_values: Option<Vec<MlSlotClassValue>>,
}
