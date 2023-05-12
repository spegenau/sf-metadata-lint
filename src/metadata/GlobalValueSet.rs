use crate::metadata::CustomValue::CustomValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GlobalValueSet  {
	#[serde(rename = "customValue")]
	pub custom_value: Option<Vec<CustomValue>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sorted")]
	pub sorted: bool,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
