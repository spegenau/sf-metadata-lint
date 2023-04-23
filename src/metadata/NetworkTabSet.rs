use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NetworkTabSet  {
	#[serde(rename = "customTab")]
	pub custom_tab: Option<Vec<String>>,
	#[serde(rename = "defaultTab")]
	pub default_tab: String,
	#[serde(rename = "standardTab")]
	pub standard_tab: Option<Vec<String>>,
}
