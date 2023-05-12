use crate::metadata::ActionLinkExecutionsAllowed::ActionLinkExecutionsAllowed;
use crate::metadata::ActionLinkTemplate::ActionLinkTemplate;
use crate::metadata::PlatformActionGroupCategory::PlatformActionGroupCategory;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionLinkGroupTemplate  {
	#[serde(rename = "actionLinkTemplates")]
	pub action_link_templates: Option<Vec<ActionLinkTemplate>>,
	#[serde(rename = "category")]
	pub category: PlatformActionGroupCategory,
	#[serde(rename = "executionsAllowed")]
	pub executions_allowed: ActionLinkExecutionsAllowed,
	#[serde(rename = "hoursUntilExpiration")]
	pub hours_until_expiration: Option<i32>,
	#[serde(rename = "isPublished")]
	pub is_published: bool,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
