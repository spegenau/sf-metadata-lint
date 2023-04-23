use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexEmailNotification  {
	#[serde(rename = "email")]
	pub email: Option<String>,
	#[serde(rename = "user")]
	pub user: Option<String>,
}
