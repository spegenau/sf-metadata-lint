use crate::metadata::RelatedContentItem::RelatedContentItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RelatedContent  {
	#[serde(rename = "relatedContentItems")]
	pub related_content_items: Option<Vec<RelatedContentItem>>,
}
