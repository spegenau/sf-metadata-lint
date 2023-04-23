use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2SettingsOpportunityFilter  {
	#[serde(rename = "apexClassName")]
	pub apex_class_name: String,
	#[serde(rename = "enableFilter")]
	pub enable_filter: bool,
	#[serde(rename = "runOnCreate")]
	pub run_on_create: bool,
}
