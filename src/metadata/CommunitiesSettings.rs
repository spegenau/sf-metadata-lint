use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunitiesSettings  {
	#[serde(rename = "applyLoginPageTypeToEmbeddedLogin")]
	pub apply_login_page_type_to_embedded_login: Option<bool>,
	#[serde(rename = "blockEmbeddedLoginUnknownURLRedirect")]
	pub block_embedded_login_unknown_url_redirect: Option<bool>,
	#[serde(rename = "canModerateAllFeedPosts")]
	pub can_moderate_all_feed_posts: Option<bool>,
	#[serde(rename = "canModerateInternalFeedPosts")]
	pub can_moderate_internal_feed_posts: Option<bool>,
	#[serde(rename = "embeddedVisualforcePages")]
	pub embedded_visualforce_pages: Option<bool>,
	#[serde(rename = "enableCommunityWorkspaces")]
	pub enable_community_workspaces: Option<bool>,
	#[serde(rename = "enableCspContactVisibilityPref")]
	pub enable_csp_contact_visibility_pref: Option<bool>,
	#[serde(rename = "enableCspNotesOnAccConPref")]
	pub enable_csp_notes_on_acc_con_pref: Option<bool>,
	#[serde(rename = "enableEnablePRM")]
	pub enable_enable_prm: Option<bool>,
	#[serde(rename = "enableExternalAccHierPref")]
	pub enable_external_acc_hier_pref: Option<bool>,
	#[serde(rename = "enableGuestPermDisOptOutCruc")]
	pub enable_guest_perm_dis_opt_out_cruc: Option<bool>,
	#[serde(rename = "enableGuestRecordReassignOrgPref")]
	pub enable_guest_record_reassign_org_pref: Option<bool>,
	#[serde(rename = "enableGuestSecurityOptOutCruc")]
	pub enable_guest_security_opt_out_cruc: Option<bool>,
	#[serde(rename = "enableGuvSecurityOptOutPref")]
	pub enable_guv_security_opt_out_pref: Option<bool>,
	#[serde(rename = "enableInviteChatterGuestEnabled")]
	pub enable_invite_chatter_guest_enabled: Option<bool>,
	#[serde(rename = "enableNameFieldsUserPIIEnabled")]
	pub enable_name_fields_user_pii_enabled: Option<bool>,
	#[serde(rename = "enableNetPortalUserReportOpts")]
	pub enable_net_portal_user_report_opts: Option<bool>,
	#[serde(rename = "enableNetworksEnabled")]
	pub enable_networks_enabled: Option<bool>,
	#[serde(rename = "enableOotbProfExtUserOpsEnable")]
	pub enable_ootb_prof_ext_user_ops_enable: Option<bool>,
	#[serde(rename = "enablePRMAccRelPref")]
	pub enable_prm_acc_rel_pref: Option<bool>,
	#[serde(rename = "enablePowerCustomerCaseStatus")]
	pub enable_power_customer_case_status: Option<bool>,
	#[serde(rename = "enablePreventBadgeGuestAccess")]
	pub enable_prevent_badge_guest_access: Option<bool>,
	#[serde(rename = "enableRelaxPartnerAccountFieldPref")]
	pub enable_relax_partner_account_field_pref: Option<bool>,
	#[serde(rename = "enableUnsupportedBrowserModalPref")]
	pub enable_unsupported_browser_modal_pref: Option<bool>,
	#[serde(rename = "enableUsernameUniqForOrgPref")]
	pub enable_username_uniq_for_org_pref: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
