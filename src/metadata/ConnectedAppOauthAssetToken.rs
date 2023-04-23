use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppOauthAssetToken  {
	#[serde(rename = "assetAudiences")]
	pub asset_audiences: String,
	#[serde(rename = "assetIncludeAttributes")]
	pub asset_include_attributes: bool,
	#[serde(rename = "assetIncludeCustomPerms")]
	pub asset_include_custom_perms: bool,
	#[serde(rename = "assetSigningCertId")]
	pub asset_signing_cert_id: String,
	#[serde(rename = "assetValidityPeriod")]
	pub asset_validity_period: i32,
}
