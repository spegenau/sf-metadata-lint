use crate::metadata::IPAddressFeature::IPAddressFeature;
use crate::metadata::IPAddressUsageScope::IPAddressUsageScope;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IPAddressRange  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "endIpAddress")]
	pub end_ip_address: String,
	#[serde(rename = "ipAddressFeature")]
	pub ip_address_feature: IPAddressFeature,
	#[serde(rename = "ipAddressUsageScope")]
	pub ip_address_usage_scope: IPAddressUsageScope,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "startIpAddress")]
	pub start_ip_address: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
