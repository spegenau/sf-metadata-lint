use crate::metadata::PicklistEntry::PicklistEntry;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValueTypeField  {
	#[serde(rename = "fields")]
	pub fields: Option<Vec<Box<ValueTypeField>>>,
	#[serde(rename = "foreignKeyDomain")]
	pub foreign_key_domain: Option<Vec<String>>,
	#[serde(rename = "isForeignKey")]
	pub is_foreign_key: bool,
	#[serde(rename = "isNameField")]
	pub is_name_field: bool,
	#[serde(rename = "minOccurs")]
	pub min_occurs: i32,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "picklistValues")]
	pub picklist_values: Option<Vec<PicklistEntry>>,
	#[serde(rename = "soapType")]
	pub soap_type: String,
	#[serde(rename = "valueRequired")]
	pub value_required: bool,
}
