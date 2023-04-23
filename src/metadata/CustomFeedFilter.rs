use crate::metadata::FeedFilterCriterion::FeedFilterCriterion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomFeedFilter  {
	#[serde(rename = "criteria")]
	pub criteria: Option<Vec<FeedFilterCriterion>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
}
