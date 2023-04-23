use crate::metadata::AccountSharingRuleSettings::AccountSharingRuleSettings;
use crate::metadata::SharedTo::SharedTo;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingBaseRule  {
	#[serde(rename = "accessLevel")]
	pub access_level: String,
	#[serde(rename = "accountSettings")]
	pub account_settings: Option<AccountSharingRuleSettings>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "sharedTo")]
	pub shared_to: SharedTo,
}
