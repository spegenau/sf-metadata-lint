use crate::metadata::SiteRedirect::SiteRedirect;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SiteRedirectMapping  {
	#[serde(rename = "action")]
	pub action: SiteRedirect,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isDynamic")]
	pub is_dynamic: Option<bool>,
	#[serde(rename = "source")]
	pub source: String,
	#[serde(rename = "target")]
	pub target: String,
}
