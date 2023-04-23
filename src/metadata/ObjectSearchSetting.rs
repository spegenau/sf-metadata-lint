use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectSearchSetting  {
	#[serde(rename = "enhancedLookupEnabled")]
	pub enhanced_lookup_enabled: bool,
	#[serde(rename = "lookupAutoCompleteEnabled")]
	pub lookup_auto_complete_enabled: bool,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "resultsPerPageCount")]
	pub results_per_page_count: i32,
}
