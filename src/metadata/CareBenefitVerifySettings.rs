use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CareBenefitVerifySettings  {
	#[serde(rename = "codeSetType")]
	pub code_set_type: Option<String>,
	#[serde(rename = "defaultNpi")]
	pub default_npi: Option<String>,
	#[serde(rename = "generalPlanServiceTypeCode")]
	pub general_plan_service_type_code: Option<String>,
	#[serde(rename = "isDefault")]
	pub is_default: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "organizationName")]
	pub organization_name: Option<String>,
	#[serde(rename = "serviceApexClass")]
	pub service_apex_class: Option<String>,
	#[serde(rename = "serviceNamedCredential")]
	pub service_named_credential: Option<String>,
	#[serde(rename = "serviceTypeSourceSystem")]
	pub service_type_source_system: Option<String>,
	#[serde(rename = "uriPath")]
	pub uri_path: Option<String>,
}
