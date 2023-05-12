use crate::metadata::NetworkUserType::NetworkUserType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UserCriteria  {
	#[serde(rename = "creationAgeInSeconds")]
	pub creation_age_in_seconds: Option<i32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "lastChatterActivityAgeInSeconds")]
	pub last_chatter_activity_age_in_seconds: Option<i32>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "profiles")]
	pub profiles: Option<Vec<String>>,
	#[serde(rename = "userTypes")]
	pub user_types: Option<Vec<NetworkUserType>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
