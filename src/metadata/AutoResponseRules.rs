use crate::metadata::AutoResponseRule::AutoResponseRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AutoResponseRules  {
	#[serde(rename = "autoResponseRule")]
	pub auto_response_rule: Option<Vec<AutoResponseRule>>,
}
