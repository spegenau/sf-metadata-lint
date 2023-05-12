use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MyDomainDiscoverableLogin  {
	#[serde(rename = "apexHandler")]
	pub apex_handler: String,
	#[serde(rename = "executeApexHandlerAs")]
	pub execute_apex_handler_as: Option<String>,
	#[serde(rename = "usernameLabel")]
	pub username_label: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
