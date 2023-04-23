use crate::metadata::SchedulingParameterKey::SchedulingParameterKey;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SchedulingRuleParameter  {
	#[serde(rename = "schedulingParameterKey")]
	pub scheduling_parameter_key: SchedulingParameterKey,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
