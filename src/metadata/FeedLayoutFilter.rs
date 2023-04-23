use crate::metadata::FeedItemType::FeedItemType;
use crate::metadata::FeedLayoutFilterType::FeedLayoutFilterType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FeedLayoutFilter  {
	#[serde(rename = "feedFilterName")]
	pub feed_filter_name: Option<String>,
	#[serde(rename = "feedFilterType")]
	pub feed_filter_type: FeedLayoutFilterType,
	#[serde(rename = "feedItemType")]
	pub feed_item_type: Option<FeedItemType>,
}
