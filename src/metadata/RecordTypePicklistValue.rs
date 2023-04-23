use crate::metadata::PicklistValue::PicklistValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordTypePicklistValue  {
	#[serde(rename = "picklist")]
	pub picklist: String,
	#[serde(rename = "values")]
	pub values: Option<Vec<PicklistValue>>,
}
