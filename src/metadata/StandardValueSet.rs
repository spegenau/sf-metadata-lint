use crate::metadata::StandardValue::StandardValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StandardValueSet  {
	#[serde(rename = "groupingStringEnum")]
	pub grouping_string_enum: Option<String>,
	#[serde(rename = "sorted")]
	pub sorted: bool,
	#[serde(rename = "standardValue")]
	pub standard_value: Option<Vec<StandardValue>>,
}
