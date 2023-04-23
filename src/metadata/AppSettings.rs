use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppSettings  {
	#[serde(rename = "connectedAppName")]
	pub connected_app_name: String,
	#[serde(rename = "enabled")]
	pub enabled: Option<bool>,
}
