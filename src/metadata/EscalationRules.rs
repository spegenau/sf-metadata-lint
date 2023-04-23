use crate::metadata::EscalationRule::EscalationRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EscalationRules  {
	#[serde(rename = "escalationRule")]
	pub escalation_rule: Option<Vec<EscalationRule>>,
}
