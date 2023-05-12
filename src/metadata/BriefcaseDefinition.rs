use crate::metadata::BriefcaseRule::BriefcaseRule;
use crate::metadata::BriefcaseType::BriefcaseType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BriefcaseDefinition  {
	#[serde(rename = "briefcaseRules")]
	pub briefcase_rules: Option<Vec<BriefcaseRule>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "type")]
	pub _type: Option<BriefcaseType>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
