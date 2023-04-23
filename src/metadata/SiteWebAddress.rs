use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SiteWebAddress  {
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "domainName")]
	pub domain_name: String,
	#[serde(rename = "primary")]
	pub primary: bool,
}
