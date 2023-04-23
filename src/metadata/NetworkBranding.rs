use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NetworkBranding  {
	#[serde(rename = "loginBackgroundImageUrl")]
	pub login_background_image_url: Option<String>,
	#[serde(rename = "loginFooterText")]
	pub login_footer_text: Option<String>,
	#[serde(rename = "loginLogo")]
	pub login_logo: Option<String>,
	#[serde(rename = "loginLogoName")]
	pub login_logo_name: Option<String>,
	#[serde(rename = "loginPrimaryColor")]
	pub login_primary_color: Option<String>,
	#[serde(rename = "loginQuaternaryColor")]
	pub login_quaternary_color: Option<String>,
	#[serde(rename = "loginRightFrameUrl")]
	pub login_right_frame_url: Option<String>,
	#[serde(rename = "network")]
	pub network: Option<String>,
	#[serde(rename = "pageFooter")]
	pub page_footer: Option<String>,
	#[serde(rename = "pageHeader")]
	pub page_header: Option<String>,
	#[serde(rename = "primaryColor")]
	pub primary_color: String,
	#[serde(rename = "primaryComplementColor")]
	pub primary_complement_color: String,
	#[serde(rename = "quaternaryColor")]
	pub quaternary_color: String,
	#[serde(rename = "quaternaryComplementColor")]
	pub quaternary_complement_color: String,
	#[serde(rename = "secondaryColor")]
	pub secondary_color: String,
	#[serde(rename = "staticLogoImageUrl")]
	pub static_logo_image_url: Option<String>,
	#[serde(rename = "tertiaryColor")]
	pub tertiary_color: String,
	#[serde(rename = "tertiaryComplementColor")]
	pub tertiary_complement_color: String,
	#[serde(rename = "zeronaryColor")]
	pub zeronary_color: String,
	#[serde(rename = "zeronaryComplementColor")]
	pub zeronary_complement_color: String,
}
