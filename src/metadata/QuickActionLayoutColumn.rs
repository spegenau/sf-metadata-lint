use crate::metadata::QuickActionLayoutItem::QuickActionLayoutItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickActionLayoutColumn  {
	#[serde(rename = "quickActionLayoutItems")]
	pub quick_action_layout_items: Option<Vec<QuickActionLayoutItem>>,
}
