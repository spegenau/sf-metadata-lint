use crate::metadata::LayoutItem::LayoutItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelatedContentItem  {
	#[serde(rename = "layoutItem")]
	pub layout_item: LayoutItem,
}
