use crate::metadata::AuthProviderType::AuthProviderType;
use crate::metadata::MuleSoftControlPlane::MuleSoftControlPlane;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AuthProvider  {
	#[serde(rename = "appleTeam")]
	pub apple_team: Option<String>,
	#[serde(rename = "authorizeUrl")]
	pub authorize_url: Option<String>,
	#[serde(rename = "consumerKey")]
	pub consumer_key: Option<String>,
	#[serde(rename = "consumerSecret")]
	pub consumer_secret: Option<String>,
	#[serde(rename = "controlPlane")]
	pub control_plane: Option<MuleSoftControlPlane>,
	#[serde(rename = "customMetadataTypeRecord")]
	pub custom_metadata_type_record: Option<String>,
	#[serde(rename = "defaultScopes")]
	pub default_scopes: Option<String>,
	#[serde(rename = "ecKey")]
	pub ec_key: Option<String>,
	#[serde(rename = "errorUrl")]
	pub error_url: Option<String>,
	#[serde(rename = "executionUser")]
	pub execution_user: Option<String>,
	#[serde(rename = "friendlyName")]
	pub friendly_name: String,
	#[serde(rename = "iconUrl")]
	pub icon_url: Option<String>,
	#[serde(rename = "idTokenIssuer")]
	pub id_token_issuer: Option<String>,
	#[serde(rename = "includeOrgIdInIdentifier")]
	pub include_org_id_in_identifier: Option<bool>,
	#[serde(rename = "linkKickoffUrl")]
	pub link_kickoff_url: Option<String>,
	#[serde(rename = "logoutUrl")]
	pub logout_url: Option<String>,
	#[serde(rename = "oauthKickoffUrl")]
	pub oauth_kickoff_url: Option<String>,
	#[serde(rename = "plugin")]
	pub plugin: Option<String>,
	#[serde(rename = "portal")]
	pub portal: Option<String>,
	#[serde(rename = "providerType")]
	pub provider_type: AuthProviderType,
	#[serde(rename = "registrationHandler")]
	pub registration_handler: Option<String>,
	#[serde(rename = "sendAccessTokenInHeader")]
	pub send_access_token_in_header: Option<bool>,
	#[serde(rename = "sendClientCredentialsInHeader")]
	pub send_client_credentials_in_header: Option<bool>,
	#[serde(rename = "sendSecretInApis")]
	pub send_secret_in_apis: Option<bool>,
	#[serde(rename = "ssoKickoffUrl")]
	pub sso_kickoff_url: Option<String>,
	#[serde(rename = "tokenUrl")]
	pub token_url: Option<String>,
	#[serde(rename = "userInfoUrl")]
	pub user_info_url: Option<String>,
}
