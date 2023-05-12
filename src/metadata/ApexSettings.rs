use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexSettings  {
	#[serde(rename = "defaultQueueableDelay")]
	pub default_queueable_delay: Option<i32>,
	#[serde(rename = "enableAggregateCodeCoverageOnly")]
	pub enable_aggregate_code_coverage_only: Option<bool>,
	#[serde(rename = "enableApexAccessRightsPref")]
	pub enable_apex_access_rights_pref: Option<bool>,
	#[serde(rename = "enableApexApprovalLockUnlock")]
	pub enable_apex_approval_lock_unlock: Option<bool>,
	#[serde(rename = "enableApexCtrlImplicitWithSharingPref")]
	pub enable_apex_ctrl_implicit_with_sharing_pref: Option<bool>,
	#[serde(rename = "enableApexPropertyGetterPref")]
	pub enable_apex_property_getter_pref: Option<bool>,
	#[serde(rename = "enableAuraApexCtrlAuthUserAccessCheckPref")]
	pub enable_aura_apex_ctrl_auth_user_access_check_pref: Option<bool>,
	#[serde(rename = "enableAuraApexCtrlGuestUserAccessCheckPref")]
	pub enable_aura_apex_ctrl_guest_user_access_check_pref: Option<bool>,
	#[serde(rename = "enableCompileOnDeploy")]
	pub enable_compile_on_deploy: Option<bool>,
	#[serde(rename = "enableDisableParallelApexTesting")]
	pub enable_disable_parallel_apex_testing: Option<bool>,
	#[serde(rename = "enableDoNotEmailDebugLog")]
	pub enable_do_not_email_debug_log: Option<bool>,
	#[serde(rename = "enableGaplessTestAutoNum")]
	pub enable_gapless_test_auto_num: Option<bool>,
	#[serde(rename = "enableMngdCtrlActionAccessPref")]
	pub enable_mngd_ctrl_action_access_pref: Option<bool>,
	#[serde(rename = "enableNonCertifiedApexMdCrud")]
	pub enable_non_certified_apex_md_crud: Option<bool>,
	#[serde(rename = "enableRestrictCommunityExecAnon")]
	pub enable_restrict_community_exec_anon: Option<bool>,
	#[serde(rename = "enableSecureNoArgConstructorPref")]
	pub enable_secure_no_arg_constructor_pref: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
