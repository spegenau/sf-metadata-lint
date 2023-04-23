use crate::metadata::ConnectedAppOauthAccessScope::ConnectedAppOauthAccessScope;
use crate::metadata::ConnectedAppOauthAssetToken::ConnectedAppOauthAssetToken;
use crate::metadata::ConnectedAppOauthIdToken::ConnectedAppOauthIdToken;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppOauthConfig  {
	#[serde(rename = "assetTokenConfig")]
	pub asset_token_config: Option<ConnectedAppOauthAssetToken>,
	#[serde(rename = "callbackUrl")]
	pub callback_url: String,
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "consumerKey")]
	pub consumer_key: Option<String>,
	#[serde(rename = "consumerSecret")]
	pub consumer_secret: Option<String>,
	#[serde(rename = "idTokenConfig")]
	pub id_token_config: Option<ConnectedAppOauthIdToken>,
	#[serde(rename = "isAdminApproved")]
	pub is_admin_approved: Option<bool>,
	#[serde(rename = "isClientCredentialEnabled")]
	pub is_client_credential_enabled: Option<bool>,
	#[serde(rename = "isCodeCredentialEnabled")]
	pub is_code_credential_enabled: Option<bool>,
	#[serde(rename = "isCodeCredentialPostOnly")]
	pub is_code_credential_post_only: Option<bool>,
	#[serde(rename = "isConsumerSecretOptional")]
	pub is_consumer_secret_optional: Option<bool>,
	#[serde(rename = "isIntrospectAllTokens")]
	pub is_introspect_all_tokens: Option<bool>,
	#[serde(rename = "isSecretRequiredForRefreshToken")]
	pub is_secret_required_for_refresh_token: Option<bool>,
	#[serde(rename = "oauthClientCredentialUser")]
	pub oauth_client_credential_user: Option<String>,
	#[serde(rename = "scopes")]
	pub scopes: Option<Vec<ConnectedAppOauthAccessScope>>,
	#[serde(rename = "singleLogoutUrl")]
	pub single_logout_url: Option<String>,
}
