use crate::metadata::FeedItemType::FeedItemType;
use crate::metadata::FeedItemVisibility::FeedItemVisibility;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FeedFilterCriterion  {
	#[serde(rename = "feedItemType")]
	pub feed_item_type: FeedItemType,
	#[serde(rename = "feedItemVisibility")]
	pub feed_item_visibility: Option<FeedItemVisibility>,
	#[serde(rename = "relatedSObjectType")]
	pub related_s_object_type: Option<String>,
}
