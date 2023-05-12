use crate::metadata::AuthenticationProtocol::AuthenticationProtocol;
use crate::metadata::ExternalCredentialParameter::ExternalCredentialParameter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalCredential  {
	#[serde(rename = "authenticationProtocol")]
	pub authentication_protocol: AuthenticationProtocol,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "externalCredentialParameters")]
	pub external_credential_parameters: Option<Vec<ExternalCredentialParameter>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
