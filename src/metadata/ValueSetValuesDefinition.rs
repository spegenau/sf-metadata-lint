use crate::metadata::CustomValue::CustomValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValueSetValuesDefinition  {
	#[serde(rename = "sorted")]
	pub sorted: bool,
	#[serde(rename = "value")]
	pub value: Option<Vec<CustomValue>>,
}
