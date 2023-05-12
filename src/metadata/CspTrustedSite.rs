use crate::metadata::CspTrustedSiteContext::CspTrustedSiteContext;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CspTrustedSite  {
	#[serde(rename = "context")]
	pub context: Option<CspTrustedSiteContext>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "endpointUrl")]
	pub endpoint_url: String,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "isApplicableToConnectSrc")]
	pub is_applicable_to_connect_src: Option<bool>,
	#[serde(rename = "isApplicableToFontSrc")]
	pub is_applicable_to_font_src: Option<bool>,
	#[serde(rename = "isApplicableToFrameSrc")]
	pub is_applicable_to_frame_src: Option<bool>,
	#[serde(rename = "isApplicableToImgSrc")]
	pub is_applicable_to_img_src: Option<bool>,
	#[serde(rename = "isApplicableToMediaSrc")]
	pub is_applicable_to_media_src: Option<bool>,
	#[serde(rename = "isApplicableToStyleSrc")]
	pub is_applicable_to_style_src: Option<bool>,
	#[serde(rename = "mobileExtension")]
	pub mobile_extension: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
