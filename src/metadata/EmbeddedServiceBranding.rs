use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceBranding  {
	#[serde(rename = "contrastInvertedColor")]
	pub contrast_inverted_color: Option<String>,
	#[serde(rename = "contrastPrimaryColor")]
	pub contrast_primary_color: Option<String>,
	#[serde(rename = "embeddedServiceConfig")]
	pub embedded_service_config: String,
	#[serde(rename = "font")]
	pub font: Option<String>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "navBarColor")]
	pub nav_bar_color: Option<String>,
	#[serde(rename = "navBarTextColor")]
	pub nav_bar_text_color: Option<String>,
	#[serde(rename = "primaryColor")]
	pub primary_color: Option<String>,
	#[serde(rename = "secondaryColor")]
	pub secondary_color: Option<String>,
	#[serde(rename = "secondaryNavBarColor")]
	pub secondary_nav_bar_color: Option<String>,
	#[serde(rename = "width")]
	pub width: Option<i32>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
