use crate::metadata::NamedCredentialParamType::NamedCredentialParamType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NamedCredentialParameter  {
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "externalCredential")]
	pub external_credential: Option<String>,
	#[serde(rename = "outboundNetworkConnection")]
	pub outbound_network_connection: Option<String>,
	#[serde(rename = "parameterName")]
	pub parameter_name: String,
	#[serde(rename = "parameterType")]
	pub parameter_type: NamedCredentialParamType,
	#[serde(rename = "parameterValue")]
	pub parameter_value: Option<String>,
	#[serde(rename = "sequenceNumber")]
	pub sequence_number: Option<i32>,
}
