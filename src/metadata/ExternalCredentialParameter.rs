use crate::metadata::ExternalCredentialParamType::ExternalCredentialParamType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalCredentialParameter  {
	#[serde(rename = "authProvider")]
	pub auth_provider: Option<String>,
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "parameterName")]
	pub parameter_name: String,
	#[serde(rename = "parameterType")]
	pub parameter_type: ExternalCredentialParamType,
	#[serde(rename = "parameterValue")]
	pub parameter_value: Option<String>,
	#[serde(rename = "principal")]
	pub principal: Option<String>,
	#[serde(rename = "sequenceNumber")]
	pub sequence_number: Option<i32>,
}
