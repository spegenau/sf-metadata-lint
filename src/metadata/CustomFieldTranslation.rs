use crate::metadata::Gender::Gender;
use crate::metadata::LookupFilterTranslation::LookupFilterTranslation;
use crate::metadata::ObjectNameCaseValue::ObjectNameCaseValue;
use crate::metadata::PicklistValueTranslation::PicklistValueTranslation;
use crate::metadata::StartsWith::StartsWith;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomFieldTranslation  {
	#[serde(rename = "caseValues")]
	pub case_values: Option<Vec<ObjectNameCaseValue>>,
	#[serde(rename = "gender")]
	pub gender: Option<Gender>,
	#[serde(rename = "help")]
	pub help: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "lookupFilter")]
	pub lookup_filter: Option<LookupFilterTranslation>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "picklistValues")]
	pub picklist_values: Option<Vec<PicklistValueTranslation>>,
	#[serde(rename = "relationshipLabel")]
	pub relationship_label: Option<String>,
	#[serde(rename = "startsWith")]
	pub starts_with: Option<StartsWith>,
}
