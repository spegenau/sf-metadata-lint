use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2RuleAssociation  {
	#[serde(rename = "inherited")]
	pub inherited: bool,
	#[serde(rename = "ruleName")]
	pub rule_name: String,
}
