use crate::metadata::NetworkAccess::NetworkAccess;
use crate::metadata::PasswordPolicies::PasswordPolicies;
use crate::metadata::SessionSettings::SessionSettings;
use crate::metadata::SingleSignOnSettings::SingleSignOnSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SecuritySettings  {
	#[serde(rename = "canUsersGrantLoginAccess")]
	pub can_users_grant_login_access: Option<bool>,
	#[serde(rename = "enableAdminLoginAsAnyUser")]
	pub enable_admin_login_as_any_user: Option<bool>,
	#[serde(rename = "enableAuditFieldsInactiveOwner")]
	pub enable_audit_fields_inactive_owner: Option<bool>,
	#[serde(rename = "enableAuraSecureEvalPref")]
	pub enable_aura_secure_eval_pref: Option<bool>,
	#[serde(rename = "enableCoepHeader")]
	pub enable_coep_header: Option<bool>,
	#[serde(rename = "enableCoopHeader")]
	pub enable_coop_header: Option<bool>,
	#[serde(rename = "enableRequireHttpsConnection")]
	pub enable_require_https_connection: Option<bool>,
	#[serde(rename = "networkAccess")]
	pub network_access: Option<NetworkAccess>,
	#[serde(rename = "passwordPolicies")]
	pub password_policies: Option<PasswordPolicies>,
	#[serde(rename = "redirectBlockModeEnabled")]
	pub redirect_block_mode_enabled: Option<bool>,
	#[serde(rename = "sessionSettings")]
	pub session_settings: Option<SessionSettings>,
	#[serde(rename = "singleSignOnSettings")]
	pub single_sign_on_settings: Option<SingleSignOnSettings>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
