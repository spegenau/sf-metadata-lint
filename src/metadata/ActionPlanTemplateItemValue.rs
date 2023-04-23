use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionPlanTemplateItemValue  {
	#[serde(rename = "itemEntityType")]
	pub item_entity_type: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "valueFormula")]
	pub value_formula: Option<String>,
	#[serde(rename = "valueLiteral")]
	pub value_literal: Option<String>,
}
