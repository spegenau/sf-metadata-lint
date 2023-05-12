use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PersonAccountOwnerPowerUser  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "portalType")]
	pub portal_type: String,
	#[serde(rename = "user")]
	pub user: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
