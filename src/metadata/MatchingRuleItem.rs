use crate::metadata::BlankValueBehavior::BlankValueBehavior;
use crate::metadata::MatchingMethod::MatchingMethod;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MatchingRuleItem  {
	#[serde(rename = "blankValueBehavior")]
	pub blank_value_behavior: Option<BlankValueBehavior>,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "matchingMethod")]
	pub matching_method: MatchingMethod,
}
