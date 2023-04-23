use crate::metadata::FeedLayoutComponentType::FeedLayoutComponentType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FeedLayoutComponent  {
	#[serde(rename = "componentType")]
	pub component_type: FeedLayoutComponentType,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "page")]
	pub page: Option<String>,
}
