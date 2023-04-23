use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SandboxSettings  {
	#[serde(rename = "disableSandboxExpirationEmails")]
	pub disable_sandbox_expiration_emails: Option<bool>,
}
