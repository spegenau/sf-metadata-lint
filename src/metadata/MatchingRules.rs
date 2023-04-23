use crate::metadata::MatchingRule::MatchingRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MatchingRules  {
	#[serde(rename = "matchingRules")]
	pub matching_rules: Option<Vec<MatchingRule>>,
}
