use crate::metadata::FieldValue::FieldValue;
use crate::metadata::Territory2AccessLevel::Territory2AccessLevel;
use crate::metadata::Territory2RuleAssociation::Territory2RuleAssociation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2  {
	#[serde(rename = "accountAccessLevel")]
	pub account_access_level: Option<String>,
	#[serde(rename = "caseAccessLevel")]
	pub case_access_level: Option<String>,
	#[serde(rename = "contactAccessLevel")]
	pub contact_access_level: Option<String>,
	#[serde(rename = "customFields")]
	pub custom_fields: Option<Vec<FieldValue>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "objectAccessLevels")]
	pub object_access_levels: Option<Vec<Territory2AccessLevel>>,
	#[serde(rename = "opportunityAccessLevel")]
	pub opportunity_access_level: Option<String>,
	#[serde(rename = "parentTerritory")]
	pub parent_territory: Option<String>,
	#[serde(rename = "ruleAssociations")]
	pub rule_associations: Option<Vec<Territory2RuleAssociation>>,
	#[serde(rename = "territory2Type")]
	pub territory_2_type: String,
}
