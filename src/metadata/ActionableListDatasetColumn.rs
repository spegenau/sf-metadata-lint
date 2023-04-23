use crate::metadata::DatasetColumnDataType::DatasetColumnDataType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionableListDatasetColumn  {
	#[serde(rename = "dataDomain")]
	pub data_domain: Option<DatasetColumnDataType>,
	#[serde(rename = "isDefault")]
	pub is_default: Option<bool>,
	#[serde(rename = "objectName")]
	pub object_name: Option<String>,
	#[serde(rename = "sourceColumnApiName")]
	pub source_column_api_name: Option<String>,
	#[serde(rename = "sourceFieldName")]
	pub source_field_name: Option<String>,
}
