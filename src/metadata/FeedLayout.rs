use crate::metadata::FeedLayoutComponent::FeedLayoutComponent;
use crate::metadata::FeedLayoutFilter::FeedLayoutFilter;
use crate::metadata::FeedLayoutFilterPosition::FeedLayoutFilterPosition;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FeedLayout  {
	#[serde(rename = "autocollapsePublisher")]
	pub autocollapse_publisher: Option<bool>,
	#[serde(rename = "compactFeed")]
	pub compact_feed: Option<bool>,
	#[serde(rename = "feedFilterPosition")]
	pub feed_filter_position: Option<FeedLayoutFilterPosition>,
	#[serde(rename = "feedFilters")]
	pub feed_filters: Option<Vec<FeedLayoutFilter>>,
	#[serde(rename = "fullWidthFeed")]
	pub full_width_feed: Option<bool>,
	#[serde(rename = "hideSidebar")]
	pub hide_sidebar: Option<bool>,
	#[serde(rename = "highlightExternalFeedItems")]
	pub highlight_external_feed_items: Option<bool>,
	#[serde(rename = "leftComponents")]
	pub left_components: Option<Vec<FeedLayoutComponent>>,
	#[serde(rename = "rightComponents")]
	pub right_components: Option<Vec<FeedLayoutComponent>>,
	#[serde(rename = "useInlineFiltersInConsole")]
	pub use_inline_filters_in_console: Option<bool>,
}
