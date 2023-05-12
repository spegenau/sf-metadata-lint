use crate::metadata::AuthenticationProtocol::AuthenticationProtocol;
use crate::metadata::ExternalPrincipalType::ExternalPrincipalType;
use crate::metadata::NamedCredentialParameter::NamedCredentialParameter;
use crate::metadata::NamedCredentialType::NamedCredentialType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NamedCredential  {
	#[serde(rename = "allowMergeFieldsInBody")]
	pub allow_merge_fields_in_body: Option<bool>,
	#[serde(rename = "allowMergeFieldsInHeader")]
	pub allow_merge_fields_in_header: Option<bool>,
	#[serde(rename = "authProvider")]
	pub auth_provider: Option<String>,
	#[serde(rename = "authTokenEndpointUrl")]
	pub auth_token_endpoint_url: Option<String>,
	#[serde(rename = "awsAccessKey")]
	pub aws_access_key: Option<String>,
	#[serde(rename = "awsAccessSecret")]
	pub aws_access_secret: Option<String>,
	#[serde(rename = "awsRegion")]
	pub aws_region: Option<String>,
	#[serde(rename = "awsService")]
	pub aws_service: Option<String>,
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "endpoint")]
	pub endpoint: Option<String>,
	#[serde(rename = "generateAuthorizationHeader")]
	pub generate_authorization_header: Option<bool>,
	#[serde(rename = "jwtAudience")]
	pub jwt_audience: Option<String>,
	#[serde(rename = "jwtFormulaSubject")]
	pub jwt_formula_subject: Option<String>,
	#[serde(rename = "jwtIssuer")]
	pub jwt_issuer: Option<String>,
	#[serde(rename = "jwtSigningCertificate")]
	pub jwt_signing_certificate: Option<String>,
	#[serde(rename = "jwtTextSubject")]
	pub jwt_text_subject: Option<String>,
	#[serde(rename = "jwtValidityPeriodSeconds")]
	pub jwt_validity_period_seconds: Option<i32>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "namedCredentialParameters")]
	pub named_credential_parameters: Option<Vec<NamedCredentialParameter>>,
	#[serde(rename = "namedCredentialType")]
	pub named_credential_type: Option<NamedCredentialType>,
	#[serde(rename = "oauthRefreshToken")]
	pub oauth_refresh_token: Option<String>,
	#[serde(rename = "oauthScope")]
	pub oauth_scope: Option<String>,
	#[serde(rename = "oauthToken")]
	pub oauth_token: Option<String>,
	#[serde(rename = "outboundNetworkConnection")]
	pub outbound_network_connection: Option<String>,
	#[serde(rename = "password")]
	pub password: Option<String>,
	#[serde(rename = "principalType")]
	pub principal_type: Option<ExternalPrincipalType>,
	#[serde(rename = "protocol")]
	pub protocol: Option<AuthenticationProtocol>,
	#[serde(rename = "username")]
	pub username: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
