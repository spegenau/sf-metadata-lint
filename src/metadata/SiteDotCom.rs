use crate::metadata::SiteType::SiteType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SiteDotCom  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "siteType")]
	pub site_type: SiteType,
}
