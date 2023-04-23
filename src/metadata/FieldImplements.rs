use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldImplements  {
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "interfaceField")]
	pub interface_field: Option<String>,
}
