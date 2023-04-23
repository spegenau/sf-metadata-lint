use crate::metadata::CleanRule::CleanRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CleanDataService  {
	#[serde(rename = "cleanRules")]
	pub clean_rules: Option<Vec<CleanRule>>,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "matchEngine")]
	pub match_engine: String,
}
