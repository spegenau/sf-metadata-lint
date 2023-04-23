use crate::metadata::RelatedListItem::RelatedListItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MiniLayout  {
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "relatedLists")]
	pub related_lists: Option<Vec<RelatedListItem>>,
}
