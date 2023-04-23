use crate::metadata::AuthenticationProtocol::AuthenticationProtocol;
use crate::metadata::CustomHttpHeader::CustomHttpHeader;
use crate::metadata::ExternalDataSourceType::ExternalDataSourceType;
use crate::metadata::ExternalDataSrcDescriptor::ExternalDataSrcDescriptor;
use crate::metadata::ExternalPrincipalType::ExternalPrincipalType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalDataSource  {
	#[serde(rename = "authProvider")]
	pub auth_provider: Option<String>,
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "customConfiguration")]
	pub custom_configuration: Option<String>,
	#[serde(rename = "customHttpHeaders")]
	pub custom_http_headers: Option<Vec<CustomHttpHeader>>,
	#[serde(rename = "endpoint")]
	pub endpoint: Option<String>,
	#[serde(rename = "externalDataSrcDescriptors")]
	pub external_data_src_descriptors: Option<Vec<ExternalDataSrcDescriptor>>,
	#[serde(rename = "isWritable")]
	pub is_writable: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "namedCredential")]
	pub named_credential: Option<String>,
	#[serde(rename = "oauthRefreshToken")]
	pub oauth_refresh_token: Option<String>,
	#[serde(rename = "oauthScope")]
	pub oauth_scope: Option<String>,
	#[serde(rename = "oauthToken")]
	pub oauth_token: Option<String>,
	#[serde(rename = "password")]
	pub password: Option<String>,
	#[serde(rename = "principalType")]
	pub principal_type: ExternalPrincipalType,
	#[serde(rename = "protocol")]
	pub protocol: AuthenticationProtocol,
	#[serde(rename = "repository")]
	pub repository: Option<String>,
	#[serde(rename = "type")]
	pub _type: ExternalDataSourceType,
	#[serde(rename = "username")]
	pub username: Option<String>,
	#[serde(rename = "version")]
	pub version: Option<String>,
}
