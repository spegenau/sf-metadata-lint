use crate::metadata::FeedItemDisplayFormat::FeedItemDisplayFormat;
use crate::metadata::FeedItemType::FeedItemType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FeedItemSettings  {
	#[serde(rename = "characterLimit")]
	pub character_limit: Option<i32>,
	#[serde(rename = "displayFormat")]
	pub display_format: Option<FeedItemDisplayFormat>,
	#[serde(rename = "feedItemType")]
	pub feed_item_type: FeedItemType,
}
