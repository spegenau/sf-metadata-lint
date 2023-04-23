use crate::metadata::DefinitionCreationType::DefinitionCreationType;
use crate::metadata::UsageTag::UsageTag;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MktDataLakeFieldAttributes  {
	#[serde(rename = "dateFormat")]
	pub date_format: Option<String>,
	#[serde(rename = "definitionCreationType")]
	pub definition_creation_type: Option<DefinitionCreationType>,
	#[serde(rename = "externalName")]
	pub external_name: Option<String>,
	#[serde(rename = "isEventDate")]
	pub is_event_date: Option<bool>,
	#[serde(rename = "isInternalOrganization")]
	pub is_internal_organization: Option<bool>,
	#[serde(rename = "isRecordModified")]
	pub is_record_modified: Option<bool>,
	#[serde(rename = "keyQualifierName")]
	pub key_qualifier_name: Option<String>,
	#[serde(rename = "mktDatalakeSrcKeyQualifier")]
	pub mkt_datalake_src_key_qualifier: Option<String>,
	#[serde(rename = "primaryIndexOrder")]
	pub primary_index_order: Option<i32>,
	#[serde(rename = "usageTag")]
	pub usage_tag: Option<UsageTag>,
}
