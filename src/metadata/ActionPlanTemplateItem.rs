use crate::metadata::ActionPlanTemplateItemValue::ActionPlanTemplateItemValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionPlanTemplateItem  {
	#[serde(rename = "actionPlanTemplateItemValue")]
	pub action_plan_template_item_value: Option<Vec<ActionPlanTemplateItemValue>>,
	#[serde(rename = "displayOrder")]
	pub display_order: Option<i32>,
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
	#[serde(rename = "itemEntityType")]
	pub item_entity_type: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "uniqueName")]
	pub unique_name: String,
}
