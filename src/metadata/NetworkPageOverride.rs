use crate::metadata::NetworkPageOverrideSetting::NetworkPageOverrideSetting;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NetworkPageOverride  {
	#[serde(rename = "changePasswordPageOverrideSetting")]
	pub change_password_page_override_setting: Option<NetworkPageOverrideSetting>,
	#[serde(rename = "forgotPasswordPageOverrideSetting")]
	pub forgot_password_page_override_setting: Option<NetworkPageOverrideSetting>,
	#[serde(rename = "homePageOverrideSetting")]
	pub home_page_override_setting: Option<NetworkPageOverrideSetting>,
	#[serde(rename = "loginPageOverrideSetting")]
	pub login_page_override_setting: Option<NetworkPageOverrideSetting>,
	#[serde(rename = "selfRegProfilePageOverrideSetting")]
	pub self_reg_profile_page_override_setting: Option<NetworkPageOverrideSetting>,
}
