use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomNotificationType  {
	#[serde(rename = "customNotifTypeName")]
	pub custom_notif_type_name: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "desktop")]
	pub desktop: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "mobile")]
	pub mobile: bool,
	#[serde(rename = "slack")]
	pub slack: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
