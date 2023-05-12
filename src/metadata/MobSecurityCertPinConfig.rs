use crate::metadata::MobileSecurityCertPinType::MobileSecurityCertPinType;
use crate::metadata::MobileSecurityMobilePlatform::MobileSecurityMobilePlatform;
use crate::metadata::MobileSecurityPolicySeverityLevel::MobileSecurityPolicySeverityLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MobSecurityCertPinConfig  {
	#[serde(rename = "certificateHash")]
	pub certificate_hash: String,
	#[serde(rename = "domainName")]
	pub domain_name: String,
	#[serde(rename = "isEnabled")]
	pub is_enabled: bool,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "isSubdomainIncluded")]
	pub is_subdomain_included: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "mobilePlatform")]
	pub mobile_platform: Option<MobileSecurityMobilePlatform>,
	#[serde(rename = "mobileSecurityAssignment")]
	pub mobile_security_assignment: Option<String>,
	#[serde(rename = "severityLevel")]
	pub severity_level: MobileSecurityPolicySeverityLevel,
	#[serde(rename = "type")]
	pub _type: MobileSecurityCertPinType,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
