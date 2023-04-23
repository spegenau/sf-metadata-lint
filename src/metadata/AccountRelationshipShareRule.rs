use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountRelationshipShareRule  {
	#[serde(rename = "accessLevel")]
	pub access_level: String,
	#[serde(rename = "accountToCriteriaField")]
	pub account_to_criteria_field: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "entityType")]
	pub entity_type: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "staticFormulaCriteria")]
	pub static_formula_criteria: Option<String>,
	#[serde(rename = "type")]
	pub _type: String,
}
