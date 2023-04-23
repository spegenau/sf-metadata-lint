use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CMSConnectLanguage  {
	#[serde(rename = "cmsLanguage")]
	pub cms_language: String,
	#[serde(rename = "language")]
	pub language: String,
}
