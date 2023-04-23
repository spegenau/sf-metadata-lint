use crate::metadata::PropertyDisplayType::PropertyDisplayType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SvcCatalogFulfillFlowItem  {
	#[serde(rename = "catalogInputVariable")]
	pub catalog_input_variable: String,
	#[serde(rename = "displayType")]
	pub display_type: Option<PropertyDisplayType>,
	#[serde(rename = "fieldDefinition")]
	pub field_definition: Option<String>,
	#[serde(rename = "fieldLookupDomain")]
	pub field_lookup_domain: Option<String>,
	#[serde(rename = "isAdditionalQuestionsInputVariable")]
	pub is_additional_questions_input_variable: Option<bool>,
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
	#[serde(rename = "lookupDomainFieldType")]
	pub lookup_domain_field_type: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "objectLookupDomain")]
	pub object_lookup_domain: Option<String>,
}
