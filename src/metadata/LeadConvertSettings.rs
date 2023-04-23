use crate::metadata::ObjectMapping::ObjectMapping;
use crate::metadata::VisibleOrRequired::VisibleOrRequired;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LeadConvertSettings  {
	#[serde(rename = "allowOwnerChange")]
	pub allow_owner_change: Option<bool>,
	#[serde(rename = "objectMapping")]
	pub object_mapping: Option<Vec<ObjectMapping>>,
	#[serde(rename = "opportunityCreationOptions")]
	pub opportunity_creation_options: Option<VisibleOrRequired>,
}
