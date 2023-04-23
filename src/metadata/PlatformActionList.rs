use crate::metadata::PlatformActionListContext::PlatformActionListContext;
use crate::metadata::PlatformActionListItem::PlatformActionListItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformActionList  {
	#[serde(rename = "actionListContext")]
	pub action_list_context: PlatformActionListContext,
	#[serde(rename = "platformActionListItems")]
	pub platform_action_list_items: Option<Vec<PlatformActionListItem>>,
	#[serde(rename = "relatedSourceEntity")]
	pub related_source_entity: Option<String>,
}
