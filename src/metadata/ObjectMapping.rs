use crate::metadata::ObjectMappingField::ObjectMappingField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectMapping  {
	#[serde(rename = "inputObject")]
	pub input_object: String,
	#[serde(rename = "mappingFields")]
	pub mapping_fields: Option<Vec<ObjectMappingField>>,
	#[serde(rename = "outputObject")]
	pub output_object: String,
}
