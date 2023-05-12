use crate::metadata::SiteClickjackProtectionLevel::SiteClickjackProtectionLevel;
use crate::metadata::SiteIframeWhiteListUrl::SiteIframeWhiteListUrl;
use crate::metadata::SiteRedirectMapping::SiteRedirectMapping;
use crate::metadata::SiteType::SiteType;
use crate::metadata::SiteWebAddress::SiteWebAddress;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomSite  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "allowHomePage")]
	pub allow_home_page: bool,
	#[serde(rename = "allowStandardAnswersPages")]
	pub allow_standard_answers_pages: Option<bool>,
	#[serde(rename = "allowStandardIdeasPages")]
	pub allow_standard_ideas_pages: bool,
	#[serde(rename = "allowStandardLookups")]
	pub allow_standard_lookups: bool,
	#[serde(rename = "allowStandardPortalPages")]
	pub allow_standard_portal_pages: bool,
	#[serde(rename = "allowStandardSearch")]
	pub allow_standard_search: bool,
	#[serde(rename = "analyticsTrackingCode")]
	pub analytics_tracking_code: Option<String>,
	#[serde(rename = "authorizationRequiredPage")]
	pub authorization_required_page: Option<String>,
	#[serde(rename = "bandwidthExceededPage")]
	pub bandwidth_exceeded_page: Option<String>,
	#[serde(rename = "browserXssProtection")]
	pub browser_xss_protection: bool,
	#[serde(rename = "cachePublicVisualforcePagesInProxyServers")]
	pub cache_public_visualforce_pages_in_proxy_servers: Option<bool>,
	#[serde(rename = "changePasswordPage")]
	pub change_password_page: Option<String>,
	#[serde(rename = "chatterAnswersForgotPasswordConfirmPage")]
	pub chatter_answers_forgot_password_confirm_page: Option<String>,
	#[serde(rename = "chatterAnswersForgotPasswordPage")]
	pub chatter_answers_forgot_password_page: Option<String>,
	#[serde(rename = "chatterAnswersHelpPage")]
	pub chatter_answers_help_page: Option<String>,
	#[serde(rename = "chatterAnswersLoginPage")]
	pub chatter_answers_login_page: Option<String>,
	#[serde(rename = "chatterAnswersRegistrationPage")]
	pub chatter_answers_registration_page: Option<String>,
	#[serde(rename = "clickjackProtectionLevel")]
	pub clickjack_protection_level: SiteClickjackProtectionLevel,
	#[serde(rename = "contentSniffingProtection")]
	pub content_sniffing_protection: bool,
	#[serde(rename = "customWebAddresses")]
	pub custom_web_addresses: Option<Vec<SiteWebAddress>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "enableAuraRequests")]
	pub enable_aura_requests: Option<bool>,
	#[serde(rename = "favoriteIcon")]
	pub favorite_icon: Option<String>,
	#[serde(rename = "fileNotFoundPage")]
	pub file_not_found_page: Option<String>,
	#[serde(rename = "forgotPasswordPage")]
	pub forgot_password_page: Option<String>,
	#[serde(rename = "genericErrorPage")]
	pub generic_error_page: Option<String>,
	#[serde(rename = "guestProfile")]
	pub guest_profile: Option<String>,
	#[serde(rename = "inMaintenancePage")]
	pub in_maintenance_page: Option<String>,
	#[serde(rename = "inactiveIndexPage")]
	pub inactive_index_page: Option<String>,
	#[serde(rename = "indexPage")]
	pub index_page: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "myProfilePage")]
	pub my_profile_page: Option<String>,
	#[serde(rename = "portal")]
	pub portal: Option<String>,
	#[serde(rename = "redirectToCustomDomain")]
	pub redirect_to_custom_domain: Option<bool>,
	#[serde(rename = "referrerPolicyOriginWhenCrossOrigin")]
	pub referrer_policy_origin_when_cross_origin: bool,
	#[serde(rename = "robotsTxtPage")]
	pub robots_txt_page: Option<String>,
	#[serde(rename = "selfRegPage")]
	pub self_reg_page: Option<String>,
	#[serde(rename = "serverIsDown")]
	pub server_is_down: Option<String>,
	#[serde(rename = "siteAdmin")]
	pub site_admin: Option<String>,
	#[serde(rename = "siteGuestRecordDefaultOwner")]
	pub site_guest_record_default_owner: Option<String>,
	#[serde(rename = "siteIframeWhiteListUrls")]
	pub site_iframe_white_list_urls: Option<Vec<SiteIframeWhiteListUrl>>,
	#[serde(rename = "siteRedirectMappings")]
	pub site_redirect_mappings: Option<Vec<SiteRedirectMapping>>,
	#[serde(rename = "siteTemplate")]
	pub site_template: Option<String>,
	#[serde(rename = "siteType")]
	pub site_type: SiteType,
	#[serde(rename = "subdomain")]
	pub subdomain: Option<String>,
	#[serde(rename = "urlPathPrefix")]
	pub url_path_prefix: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
