use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectMappingField  {
	#[serde(rename = "inputField")]
	pub input_field: String,
	#[serde(rename = "outputField")]
	pub output_field: String,
}
