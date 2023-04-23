use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppOauthPolicy  {
	#[serde(rename = "ipRelaxation")]
	pub ip_relaxation: String,
	#[serde(rename = "refreshTokenPolicy")]
	pub refresh_token_policy: String,
	#[serde(rename = "singleLogoutUrl")]
	pub single_logout_url: Option<String>,
}
