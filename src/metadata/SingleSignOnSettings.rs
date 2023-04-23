use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SingleSignOnSettings  {
	#[serde(rename = "enableCaseInsensitiveFederationID")]
	pub enable_case_insensitive_federation_id: Option<bool>,
	#[serde(rename = "enableForceDelegatedCallout")]
	pub enable_force_delegated_callout: Option<bool>,
	#[serde(rename = "enableMultipleSamlConfigs")]
	pub enable_multiple_saml_configs: Option<bool>,
	#[serde(rename = "enableSamlJitProvisioning")]
	pub enable_saml_jit_provisioning: Option<bool>,
	#[serde(rename = "enableSamlLogin")]
	pub enable_saml_login: Option<bool>,
	#[serde(rename = "isLoginWithSalesforceCredentialsDisabled")]
	pub is_login_with_salesforce_credentials_disabled: Option<bool>,
}
