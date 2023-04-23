use crate::metadata::LayoutItem::LayoutItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LayoutColumn  {
	#[serde(rename = "layoutItems")]
	pub layout_items: Option<Vec<LayoutItem>>,
	#[serde(rename = "reserved")]
	pub reserved: Option<String>,
}
