use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningExperienceSettings  {
	#[serde(rename = "activeThemeName")]
	pub active_theme_name: Option<String>,
	#[serde(rename = "enableAccessCheckCrucPref")]
	pub enable_access_check_cruc_pref: Option<bool>,
	#[serde(rename = "enableApiUserLtngOutAccessPref")]
	pub enable_api_user_ltng_out_access_pref: Option<bool>,
	#[serde(rename = "enableAuraCDNPref")]
	pub enable_aura_cdn_pref: Option<bool>,
	#[serde(rename = "enableAuraSecStaticResCRUCPref")]
	pub enable_aura_sec_static_res_cruc_pref: Option<bool>,
	#[serde(rename = "enableErrorExperienceEnabled")]
	pub enable_error_experience_enabled: Option<bool>,
	#[serde(rename = "enableFeedbackInMobile")]
	pub enable_feedback_in_mobile: Option<bool>,
	#[serde(rename = "enableGoogleSheetsForSfdcEnabled")]
	pub enable_google_sheets_for_sfdc_enabled: Option<bool>,
	#[serde(rename = "enableHideOpenInQuip")]
	pub enable_hide_open_in_quip: Option<bool>,
	#[serde(rename = "enableIE11DeprecationMsgHidden")]
	pub enable_ie_11_deprecation_msg_hidden: Option<bool>,
	#[serde(rename = "enableIE11LEXCrucPref")]
	pub enable_ie_11_lex_cruc_pref: Option<bool>,
	#[serde(rename = "enableInAppLearning")]
	pub enable_in_app_learning: Option<bool>,
	#[serde(rename = "enableInAppTooltips")]
	pub enable_in_app_tooltips: Option<bool>,
	#[serde(rename = "enableLEXExtensionComponentCustomization")]
	pub enable_lex_extension_component_customization: Option<bool>,
	#[serde(rename = "enableLEXExtensionDarkMode")]
	pub enable_lex_extension_dark_mode: Option<bool>,
	#[serde(rename = "enableLEXExtensionInlineEditModifier")]
	pub enable_lex_extension_inline_edit_modifier: Option<bool>,
	#[serde(rename = "enableLEXExtensionLinkGrabber")]
	pub enable_lex_extension_link_grabber: Option<bool>,
	#[serde(rename = "enableLEXExtensionRelatedLists")]
	pub enable_lex_extension_related_lists: Option<bool>,
	#[serde(rename = "enableLEXExtensionRequiredFields")]
	pub enable_lex_extension_required_fields: Option<bool>,
	#[serde(rename = "enableLEXExtensionTrailhead")]
	pub enable_lex_extension_trailhead: Option<bool>,
	#[serde(rename = "enableLEXOnIpadEnabled")]
	pub enable_lex_on_ipad_enabled: Option<bool>,
	#[serde(rename = "enableLWCDynamicComponents")]
	pub enable_lwc_dynamic_components: Option<bool>,
	#[serde(rename = "enableLexEndUsersNoSwitching")]
	pub enable_lex_end_users_no_switching: Option<bool>,
	#[serde(rename = "enableNavPersonalizationOptOut")]
	pub enable_nav_personalization_opt_out: Option<bool>,
	#[serde(rename = "enableNoBackgroundNavigations")]
	pub enable_no_background_navigations: Option<bool>,
	#[serde(rename = "enableQuip")]
	pub enable_quip: Option<bool>,
	#[serde(rename = "enableRemoveThemeBrandBanner")]
	pub enable_remove_theme_brand_banner: Option<bool>,
	#[serde(rename = "enableS1BannerPref")]
	pub enable_s_1_banner_pref: Option<bool>,
	#[serde(rename = "enableS1BrowserEnabled")]
	pub enable_s_1_browser_enabled: Option<bool>,
	#[serde(rename = "enableS1DesktopEnabled")]
	pub enable_s_1_desktop_enabled: Option<bool>,
	#[serde(rename = "enableS1UiLoggingEnabled")]
	pub enable_s_1_ui_logging_enabled: Option<bool>,
	#[serde(rename = "enableSalesforceNext")]
	pub enable_salesforce_next: Option<bool>,
	#[serde(rename = "enableSidToken3rdPartyAuraApp")]
	pub enable_sid_token_3_rd_party_aura_app: Option<bool>,
	#[serde(rename = "enableSkypeChatEnabled")]
	pub enable_skype_chat_enabled: Option<bool>,
	#[serde(rename = "enableSparkAllUsers")]
	pub enable_spark_all_users: Option<bool>,
	#[serde(rename = "enableSparkConversationEnabled")]
	pub enable_spark_conversation_enabled: Option<bool>,
	#[serde(rename = "enableTryLightningOptOut")]
	pub enable_try_lightning_opt_out: Option<bool>,
	#[serde(rename = "enableUseS1AlohaDesktop")]
	pub enable_use_s_1_aloha_desktop: Option<bool>,
	#[serde(rename = "enableUsersAreLightningOnly")]
	pub enable_users_are_lightning_only: Option<bool>,
	#[serde(rename = "enableWebExEnabled")]
	pub enable_web_ex_enabled: Option<bool>,
	#[serde(rename = "enableWebexAllUsers")]
	pub enable_webex_all_users: Option<bool>,
	#[serde(rename = "isLEXExtensionComponentCustomizationOff")]
	pub is_lex_extension_component_customization_off: Option<bool>,
	#[serde(rename = "isLEXExtensionDarkModeOff")]
	pub is_lex_extension_dark_mode_off: Option<bool>,
	#[serde(rename = "isLEXExtensionLinkGrabberOff")]
	pub is_lex_extension_link_grabber_off: Option<bool>,
	#[serde(rename = "isLEXExtensionOff")]
	pub is_lex_extension_off: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
