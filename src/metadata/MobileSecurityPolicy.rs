use crate::metadata::MobileSecurityMobilePlatform::MobileSecurityMobilePlatform;
use crate::metadata::MobileSecurityPolicyRuleValueType::MobileSecurityPolicyRuleValueType;
use crate::metadata::MobileSecurityPolicySeverityLevel::MobileSecurityPolicySeverityLevel;
use crate::metadata::MobileSecurityPolicyType::MobileSecurityPolicyType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MobileSecurityPolicy  {
	#[serde(rename = "effectiveDate")]
	pub effective_date: Option<String>,
	#[serde(rename = "isEnabled")]
	pub is_enabled: bool,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "mobilePlatform")]
	pub mobile_platform: Option<MobileSecurityMobilePlatform>,
	#[serde(rename = "mobileSecurityAssignment")]
	pub mobile_security_assignment: Option<String>,
	#[serde(rename = "ruleValue")]
	pub rule_value: String,
	#[serde(rename = "ruleValueType")]
	pub rule_value_type: MobileSecurityPolicyRuleValueType,
	#[serde(rename = "severityLevel")]
	pub severity_level: MobileSecurityPolicySeverityLevel,
	#[serde(rename = "type")]
	pub _type: MobileSecurityPolicyType,
}
