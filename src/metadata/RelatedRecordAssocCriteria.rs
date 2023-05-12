use crate::metadata::AssociationEventType::AssociationEventType;
use crate::metadata::AssociationStatusType::AssociationStatusType;
use crate::metadata::AssociationType::AssociationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelatedRecordAssocCriteria  {
	#[serde(rename = "associationHandlerApexClass")]
	pub association_handler_apex_class: Option<String>,
	#[serde(rename = "associationType")]
	pub association_type: AssociationType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "eventType")]
	pub event_type: AssociationEventType,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "preCondition")]
	pub pre_condition: String,
	#[serde(rename = "referenceObject")]
	pub reference_object: String,
	#[serde(rename = "selectedOwnerField")]
	pub selected_owner_field: Option<String>,
	#[serde(rename = "status")]
	pub status: AssociationStatusType,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
