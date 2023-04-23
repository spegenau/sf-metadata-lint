use crate::metadata::ActionPlanTemplateItem::ActionPlanTemplateItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionPlanTemplateItemDependency  {
	#[serde(rename = "creationType")]
	pub creation_type: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "previousTemplateItem")]
	pub previous_template_item: ActionPlanTemplateItem,
	#[serde(rename = "templateItem")]
	pub template_item: ActionPlanTemplateItem,
}
