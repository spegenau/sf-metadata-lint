use crate::metadata::PicklistValue::PicklistValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BusinessProcess  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "values")]
	pub values: Option<Vec<PicklistValue>>,
}
