use crate::metadata::ActionPlanTemplateItem::ActionPlanTemplateItem;
use crate::metadata::ActionPlanTemplateItemDependency::ActionPlanTemplateItemDependency;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionPlanTemplate  {
	#[serde(rename = "actionPlanTemplateItem")]
	pub action_plan_template_item: Option<Vec<ActionPlanTemplateItem>>,
	#[serde(rename = "actionPlanTemplateItemDependencies")]
	pub action_plan_template_item_dependencies: Option<Vec<ActionPlanTemplateItemDependency>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isAdHocItemCreationEnabled")]
	pub is_ad_hoc_item_creation_enabled: bool,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "targetEntityType")]
	pub target_entity_type: String,
	#[serde(rename = "uniqueName")]
	pub unique_name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
