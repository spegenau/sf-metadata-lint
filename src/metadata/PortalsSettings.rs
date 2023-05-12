use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PortalsSettings  {
	#[serde(rename = "clickjackSSPLoginPage")]
	pub clickjack_ssp_login_page: Option<bool>,
	#[serde(rename = "redirectPortalLoginToHttps")]
	pub redirect_portal_login_to_https: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
