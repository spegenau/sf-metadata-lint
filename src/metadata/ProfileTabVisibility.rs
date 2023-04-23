use crate::metadata::TabVisibility::TabVisibility;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileTabVisibility  {
	#[serde(rename = "tab")]
	pub tab: String,
	#[serde(rename = "visibility")]
	pub visibility: TabVisibility,
}
