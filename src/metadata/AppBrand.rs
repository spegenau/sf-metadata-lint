use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppBrand  {
	#[serde(rename = "footerColor")]
	pub footer_color: Option<String>,
	#[serde(rename = "headerColor")]
	pub header_color: Option<String>,
	#[serde(rename = "logo")]
	pub logo: Option<String>,
	#[serde(rename = "logoVersion")]
	pub logo_version: Option<i32>,
	#[serde(rename = "shouldOverrideOrgTheme")]
	pub should_override_org_theme: Option<bool>,
}
