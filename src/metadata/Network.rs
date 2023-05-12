use crate::metadata::CommunityRoles::CommunityRoles;
use crate::metadata::NavigationLinkSet::NavigationLinkSet;
use crate::metadata::NetworkMemberGroup::NetworkMemberGroup;
use crate::metadata::NetworkPageOverride::NetworkPageOverride;
use crate::metadata::NetworkStatus::NetworkStatus;
use crate::metadata::NetworkTabSet::NetworkTabSet;
use crate::metadata::RecommendationAudience::RecommendationAudience;
use crate::metadata::RecommendationDefinition::RecommendationDefinition;
use crate::metadata::ReputationLevelDefinitions::ReputationLevelDefinitions;
use crate::metadata::ReputationPointsRules::ReputationPointsRules;
use crate::metadata::SitesArchiveStatus::SitesArchiveStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Network  {
	#[serde(rename = "allowInternalUserLogin")]
	pub allow_internal_user_login: Option<bool>,
	#[serde(rename = "allowMembersToFlag")]
	pub allow_members_to_flag: Option<bool>,
	#[serde(rename = "allowedExtensions")]
	pub allowed_extensions: Option<String>,
	#[serde(rename = "caseCommentEmailTemplate")]
	pub case_comment_email_template: Option<String>,
	#[serde(rename = "changePasswordTemplate")]
	pub change_password_template: String,
	#[serde(rename = "chgEmailVerNewTemplate")]
	pub chg_email_ver_new_template: Option<String>,
	#[serde(rename = "chgEmailVerOldTemplate")]
	pub chg_email_ver_old_template: Option<String>,
	#[serde(rename = "communityRoles")]
	pub community_roles: Option<CommunityRoles>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "deviceActEmailTemplate")]
	pub device_act_email_template: Option<String>,
	#[serde(rename = "disableReputationRecordConversations")]
	pub disable_reputation_record_conversations: Option<bool>,
	#[serde(rename = "emailFooterLogo")]
	pub email_footer_logo: Option<String>,
	#[serde(rename = "emailFooterText")]
	pub email_footer_text: Option<String>,
	#[serde(rename = "emailSenderAddress")]
	pub email_sender_address: String,
	#[serde(rename = "emailSenderName")]
	pub email_sender_name: String,
	#[serde(rename = "enableApexCDNCaching")]
	pub enable_apex_cdn_caching: Option<bool>,
	#[serde(rename = "enableCustomVFErrorPageOverrides")]
	pub enable_custom_vf_error_page_overrides: Option<bool>,
	#[serde(rename = "enableDirectMessages")]
	pub enable_direct_messages: Option<bool>,
	#[serde(rename = "enableExperienceBundleBasedSnaOverrideEnabled")]
	pub enable_experience_bundle_based_sna_override_enabled: Option<bool>,
	#[serde(rename = "enableGuestChatter")]
	pub enable_guest_chatter: Option<bool>,
	#[serde(rename = "enableGuestFileAccess")]
	pub enable_guest_file_access: Option<bool>,
	#[serde(rename = "enableGuestMemberVisibility")]
	pub enable_guest_member_visibility: Option<bool>,
	#[serde(rename = "enableImageOptimizationCDN")]
	pub enable_image_optimization_cdn: Option<bool>,
	#[serde(rename = "enableInvitation")]
	pub enable_invitation: Option<bool>,
	#[serde(rename = "enableKnowledgeable")]
	pub enable_knowledgeable: Option<bool>,
	#[serde(rename = "enableMemberVisibility")]
	pub enable_member_visibility: Option<bool>,
	#[serde(rename = "enableNicknameDisplay")]
	pub enable_nickname_display: Option<bool>,
	#[serde(rename = "enablePrivateMessages")]
	pub enable_private_messages: Option<bool>,
	#[serde(rename = "enableReputation")]
	pub enable_reputation: Option<bool>,
	#[serde(rename = "enableShowAllNetworkSettings")]
	pub enable_show_all_network_settings: Option<bool>,
	#[serde(rename = "enableSiteAsContainer")]
	pub enable_site_as_container: Option<bool>,
	#[serde(rename = "enableTalkingAboutStats")]
	pub enable_talking_about_stats: Option<bool>,
	#[serde(rename = "enableTopicAssignmentRules")]
	pub enable_topic_assignment_rules: Option<bool>,
	#[serde(rename = "enableTopicSuggestions")]
	pub enable_topic_suggestions: Option<bool>,
	#[serde(rename = "enableUpDownVote")]
	pub enable_up_down_vote: Option<bool>,
	#[serde(rename = "feedChannel")]
	pub feed_channel: Option<String>,
	#[serde(rename = "forgotPasswordTemplate")]
	pub forgot_password_template: String,
	#[serde(rename = "gatherCustomerSentimentData")]
	pub gather_customer_sentiment_data: Option<bool>,
	#[serde(rename = "headlessForgotPasswordTemplate")]
	pub headless_forgot_password_template: Option<String>,
	#[serde(rename = "lockoutTemplate")]
	pub lockout_template: Option<String>,
	#[serde(rename = "logoutUrl")]
	pub logout_url: Option<String>,
	#[serde(rename = "maxFileSizeKb")]
	pub max_file_size_kb: Option<i32>,
	#[serde(rename = "navigationLinkSet")]
	pub navigation_link_set: Option<NavigationLinkSet>,
	#[serde(rename = "networkMemberGroups")]
	pub network_member_groups: Option<NetworkMemberGroup>,
	#[serde(rename = "networkPageOverrides")]
	pub network_page_overrides: Option<NetworkPageOverride>,
	#[serde(rename = "newSenderAddress")]
	pub new_sender_address: Option<String>,
	#[serde(rename = "picassoSite")]
	pub picasso_site: Option<String>,
	#[serde(rename = "recommendationAudience")]
	pub recommendation_audience: Option<RecommendationAudience>,
	#[serde(rename = "recommendationDefinition")]
	pub recommendation_definition: Option<RecommendationDefinition>,
	#[serde(rename = "reputationLevels")]
	pub reputation_levels: Option<ReputationLevelDefinitions>,
	#[serde(rename = "reputationPointsRules")]
	pub reputation_points_rules: Option<ReputationPointsRules>,
	#[serde(rename = "selfRegMicroBatchSubErrorEmailTemplate")]
	pub self_reg_micro_batch_sub_error_email_template: Option<String>,
	#[serde(rename = "selfRegProfile")]
	pub self_reg_profile: Option<String>,
	#[serde(rename = "selfRegistration")]
	pub self_registration: Option<bool>,
	#[serde(rename = "sendWelcomeEmail")]
	pub send_welcome_email: Option<bool>,
	#[serde(rename = "site")]
	pub site: String,
	#[serde(rename = "siteArchiveStatus")]
	pub site_archive_status: Option<SitesArchiveStatus>,
	#[serde(rename = "status")]
	pub status: NetworkStatus,
	#[serde(rename = "tabs")]
	pub tabs: NetworkTabSet,
	#[serde(rename = "urlPathPrefix")]
	pub url_path_prefix: Option<String>,
	#[serde(rename = "verificationTemplate")]
	pub verification_template: Option<String>,
	#[serde(rename = "welcomeTemplate")]
	pub welcome_template: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
