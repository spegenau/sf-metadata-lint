use crate::metadata::QuickActionListItem::QuickActionListItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickActionList  {
	#[serde(rename = "quickActionListItems")]
	pub quick_action_list_items: Option<Vec<QuickActionListItem>>,
}
