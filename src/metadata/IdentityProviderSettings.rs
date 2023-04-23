use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdentityProviderSettings  {
	#[serde(rename = "certificateName")]
	pub certificate_name: String,
	#[serde(rename = "enableIdentityProvider")]
	pub enable_identity_provider: bool,
}
