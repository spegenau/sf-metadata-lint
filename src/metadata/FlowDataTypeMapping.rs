use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowDataTypeMapping  {
	#[serde(rename = "typeName")]
	pub type_name: String,
	#[serde(rename = "typeValue")]
	pub type_value: String,
}
