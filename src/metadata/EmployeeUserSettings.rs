use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmployeeUserSettings  {
	#[serde(rename = "emailEncoding")]
	pub email_encoding: String,
	#[serde(rename = "enableEmployeeAutoCreateUser")]
	pub enable_employee_auto_create_user: Option<bool>,
	#[serde(rename = "enableEmployeeIsSourceOfTruth")]
	pub enable_employee_is_source_of_truth: Option<bool>,
	#[serde(rename = "permset")]
	pub permset: Option<String>,
	#[serde(rename = "profile")]
	pub profile: String,
	#[serde(rename = "usernameSuffix")]
	pub username_suffix: Option<String>,
}
