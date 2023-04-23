use crate::metadata::MappingType::MappingType;
use crate::metadata::ObjectMapping::ObjectMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectMappingItem  {
	#[serde(rename = "mappingType")]
	pub mapping_type: MappingType,
	#[serde(rename = "objectMapping")]
	pub object_mapping: ObjectMapping,
}
