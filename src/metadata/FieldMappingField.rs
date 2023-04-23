use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldMappingField  {
	#[serde(rename = "dataServiceField")]
	pub data_service_field: String,
	#[serde(rename = "dataServiceObjectName")]
	pub data_service_object_name: String,
	#[serde(rename = "priority")]
	pub priority: i32,
}
