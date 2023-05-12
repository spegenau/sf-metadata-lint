use crate::metadata::SamlIdentityLocationType::SamlIdentityLocationType;
use crate::metadata::SamlIdentityType::SamlIdentityType;
use crate::metadata::SamlSpSLOBinding::SamlSpSLOBinding;
use crate::metadata::SamlType::SamlType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SamlSsoConfig  {
	#[serde(rename = "attributeName")]
	pub attribute_name: Option<String>,
	#[serde(rename = "attributeNameIdFormat")]
	pub attribute_name_id_format: Option<String>,
	#[serde(rename = "decryptionCertificate")]
	pub decryption_certificate: Option<String>,
	#[serde(rename = "errorUrl")]
	pub error_url: Option<String>,
	#[serde(rename = "executionUserId")]
	pub execution_user_id: Option<String>,
	#[serde(rename = "identityLocation")]
	pub identity_location: SamlIdentityLocationType,
	#[serde(rename = "identityMapping")]
	pub identity_mapping: SamlIdentityType,
	#[serde(rename = "issuer")]
	pub issuer: String,
	#[serde(rename = "loginUrl")]
	pub login_url: Option<String>,
	#[serde(rename = "logoutUrl")]
	pub logout_url: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "oauthTokenEndpoint")]
	pub oauth_token_endpoint: Option<String>,
	#[serde(rename = "redirectBinding")]
	pub redirect_binding: Option<bool>,
	#[serde(rename = "requestSignatureMethod")]
	pub request_signature_method: Option<String>,
	#[serde(rename = "requestSigningCertId")]
	pub request_signing_cert_id: Option<String>,
	#[serde(rename = "salesforceLoginUrl")]
	pub salesforce_login_url: Option<String>,
	#[serde(rename = "samlEntityId")]
	pub saml_entity_id: String,
	#[serde(rename = "samlJitHandlerId")]
	pub saml_jit_handler_id: Option<String>,
	#[serde(rename = "samlVersion")]
	pub saml_version: SamlType,
	#[serde(rename = "singleLogoutBinding")]
	pub single_logout_binding: Option<SamlSpSLOBinding>,
	#[serde(rename = "singleLogoutUrl")]
	pub single_logout_url: Option<String>,
	#[serde(rename = "useConfigRequestMethod")]
	pub use_config_request_method: Option<bool>,
	#[serde(rename = "useSameDigestAlgoForSigning")]
	pub use_same_digest_algo_for_signing: Option<bool>,
	#[serde(rename = "userProvisioning")]
	pub user_provisioning: Option<bool>,
	#[serde(rename = "validationCert")]
	pub validation_cert: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
