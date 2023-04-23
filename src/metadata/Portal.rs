use crate::metadata::PortalRoles::PortalRoles;
use crate::metadata::PortalType::PortalType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Portal  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "admin")]
	pub admin: Option<String>,
	#[serde(rename = "defaultLanguage")]
	pub default_language: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "emailSenderAddress")]
	pub email_sender_address: String,
	#[serde(rename = "emailSenderName")]
	pub email_sender_name: String,
	#[serde(rename = "enableSelfCloseCase")]
	pub enable_self_close_case: Option<bool>,
	#[serde(rename = "footerDocument")]
	pub footer_document: Option<String>,
	#[serde(rename = "forgotPassTemplate")]
	pub forgot_pass_template: Option<String>,
	#[serde(rename = "headerDocument")]
	pub header_document: Option<String>,
	#[serde(rename = "isSelfRegistrationActivated")]
	pub is_self_registration_activated: Option<bool>,
	#[serde(rename = "loginHeaderDocument")]
	pub login_header_document: Option<String>,
	#[serde(rename = "logoDocument")]
	pub logo_document: Option<String>,
	#[serde(rename = "logoutUrl")]
	pub logout_url: Option<String>,
	#[serde(rename = "newCommentTemplate")]
	pub new_comment_template: Option<String>,
	#[serde(rename = "newPassTemplate")]
	pub new_pass_template: Option<String>,
	#[serde(rename = "newUserTemplate")]
	pub new_user_template: Option<String>,
	#[serde(rename = "ownerNotifyTemplate")]
	pub owner_notify_template: Option<String>,
	#[serde(rename = "selfRegNewUserUrl")]
	pub self_reg_new_user_url: Option<String>,
	#[serde(rename = "selfRegUserDefaultProfile")]
	pub self_reg_user_default_profile: Option<String>,
	#[serde(rename = "selfRegUserDefaultRole")]
	pub self_reg_user_default_role: Option<PortalRoles>,
	#[serde(rename = "selfRegUserTemplate")]
	pub self_reg_user_template: Option<String>,
	#[serde(rename = "showActionConfirmation")]
	pub show_action_confirmation: Option<bool>,
	#[serde(rename = "stylesheetDocument")]
	pub stylesheet_document: Option<String>,
	#[serde(rename = "type")]
	pub _type: PortalType,
}
