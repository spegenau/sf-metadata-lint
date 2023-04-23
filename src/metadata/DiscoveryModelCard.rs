use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryModelCard  {
	#[serde(rename = "contactEmail")]
	pub contact_email: Option<String>,
	#[serde(rename = "contactName")]
	pub contact_name: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "sections")]
	pub sections: Option<String>,
}
