use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SiteSettings  {
	#[serde(rename = "enableEnhancedSitesAndContentPlatform")]
	pub enable_enhanced_sites_and_content_platform: Option<bool>,
	#[serde(rename = "enableProxyLoginICHeader")]
	pub enable_proxy_login_ic_header: Option<bool>,
	#[serde(rename = "enableSitesRecordReassignOrgPref")]
	pub enable_sites_record_reassign_org_pref: Option<bool>,
	#[serde(rename = "enableTopicsInSites")]
	pub enable_topics_in_sites: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
