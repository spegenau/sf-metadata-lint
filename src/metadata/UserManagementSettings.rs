use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UserManagementSettings  {
	#[serde(rename = "enableCanAnswerContainUsername")]
	pub enable_can_answer_contain_username: Option<bool>,
	#[serde(rename = "enableConcealPersonalInfo")]
	pub enable_conceal_personal_info: Option<bool>,
	#[serde(rename = "enableContactlessExternalIdentityUsers")]
	pub enable_contactless_external_identity_users: Option<bool>,
	#[serde(rename = "enableEnhancedConcealPersonalInfo")]
	pub enable_enhanced_conceal_personal_info: Option<bool>,
	#[serde(rename = "enableEnhancedPermsetMgmt")]
	pub enable_enhanced_permset_mgmt: Option<bool>,
	#[serde(rename = "enableEnhancedProfileMgmt")]
	pub enable_enhanced_profile_mgmt: Option<bool>,
	#[serde(rename = "enableNewProfileUI")]
	pub enable_new_profile_ui: Option<bool>,
	#[serde(rename = "enableProfileFiltering")]
	pub enable_profile_filtering: Option<bool>,
	#[serde(rename = "enableRestrictEmailDomains")]
	pub enable_restrict_email_domains: Option<bool>,
	#[serde(rename = "enableScrambleUserData")]
	pub enable_scramble_user_data: Option<bool>,
	#[serde(rename = "enableUserSelfDeactivate")]
	pub enable_user_self_deactivate: Option<bool>,
	#[serde(rename = "permsetsInFieldCreation")]
	pub permsets_in_field_creation: Option<bool>,
	#[serde(rename = "psaExpirationUIEnabled")]
	pub psa_expiration_ui_enabled: Option<bool>,
	#[serde(rename = "restrictedProfileCloning")]
	pub restricted_profile_cloning: Option<bool>,
}
