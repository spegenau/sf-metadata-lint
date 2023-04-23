use crate::metadata::UiBehavior::UiBehavior;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickActionLayoutItem  {
	#[serde(rename = "emptySpace")]
	pub empty_space: Option<bool>,
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "uiBehavior")]
	pub ui_behavior: Option<UiBehavior>,
}
