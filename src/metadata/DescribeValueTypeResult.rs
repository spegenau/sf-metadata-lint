use crate::metadata::ValueTypeField::ValueTypeField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DescribeValueTypeResult  {
	#[serde(rename = "apiCreatable")]
	pub api_creatable: bool,
	#[serde(rename = "apiDeletable")]
	pub api_deletable: bool,
	#[serde(rename = "apiReadable")]
	pub api_readable: bool,
	#[serde(rename = "apiUpdatable")]
	pub api_updatable: bool,
	#[serde(rename = "parentField")]
	pub parent_field: Option<ValueTypeField>,
	#[serde(rename = "valueTypeFields")]
	pub value_type_fields: Option<Vec<ValueTypeField>>,
}
