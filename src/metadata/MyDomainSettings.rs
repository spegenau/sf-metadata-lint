use crate::metadata::OrgDomainProdSuffix::OrgDomainProdSuffix;
use crate::metadata::OrgDomainShard::OrgDomainShard;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MyDomainSettings  {
	#[serde(rename = "canOnlyLoginWithMyDomainUrl")]
	pub can_only_login_with_my_domain_url: Option<bool>,
	#[serde(rename = "doesApiLoginRequireOrgDomain")]
	pub does_api_login_require_org_domain: Option<bool>,
	#[serde(rename = "domainPartition")]
	pub domain_partition: Option<OrgDomainShard>,
	#[serde(rename = "enableNativeBrowserForAuthOnAndroid")]
	pub enable_native_browser_for_auth_on_android: Option<bool>,
	#[serde(rename = "enableNativeBrowserForAuthOnIos")]
	pub enable_native_browser_for_auth_on_ios: Option<bool>,
	#[serde(rename = "enableShareBrowserSessionAndroidForAuth")]
	pub enable_share_browser_session_android_for_auth: Option<bool>,
	#[serde(rename = "enableShareBrowserSessionIOSForAuth")]
	pub enable_share_browser_session_ios_for_auth: Option<bool>,
	#[serde(rename = "logRedirections")]
	pub log_redirections: Option<bool>,
	#[serde(rename = "myDomainName")]
	pub my_domain_name: Option<String>,
	#[serde(rename = "myDomainSuffix")]
	pub my_domain_suffix: Option<OrgDomainProdSuffix>,
	#[serde(rename = "redirectForceComSiteUrls")]
	pub redirect_force_com_site_urls: Option<bool>,
	#[serde(rename = "redirectPriorMyDomain")]
	pub redirect_prior_my_domain: Option<bool>,
	#[serde(rename = "use3rdPartyCookieBlockingCompatibleHostnames")]
	pub use_3_rd_party_cookie_blocking_compatible_hostnames: Option<bool>,
	#[serde(rename = "useEdge")]
	pub use_edge: Option<bool>,
	#[serde(rename = "useEnhancedDomainsInSandbox")]
	pub use_enhanced_domains_in_sandbox: Option<bool>,
	#[serde(rename = "useStabilizedMyDomainHostnames")]
	pub use_stabilized_my_domain_hostnames: Option<bool>,
	#[serde(rename = "useStabilizedSandboxMyDomainHostnames")]
	pub use_stabilized_sandbox_my_domain_hostnames: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
