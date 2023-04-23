use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppOauthIdToken  {
	#[serde(rename = "idTokenAudience")]
	pub id_token_audience: Option<String>,
	#[serde(rename = "idTokenIncludeAttributes")]
	pub id_token_include_attributes: Option<bool>,
	#[serde(rename = "idTokenIncludeCustomPerms")]
	pub id_token_include_custom_perms: Option<bool>,
	#[serde(rename = "idTokenIncludeStandardClaims")]
	pub id_token_include_standard_claims: Option<bool>,
	#[serde(rename = "idTokenValidity")]
	pub id_token_validity: Option<i32>,
}
