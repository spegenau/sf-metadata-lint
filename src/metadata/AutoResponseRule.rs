use crate::metadata::RuleEntry::RuleEntry;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AutoResponseRule  {
	#[serde(rename = "active")]
	pub active: Option<bool>,
	#[serde(rename = "ruleEntry")]
	pub rule_entry: Option<Vec<RuleEntry>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
