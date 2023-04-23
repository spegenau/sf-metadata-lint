use crate::metadata::CMSConnectAsset::CMSConnectAsset;
use crate::metadata::CMSConnectLanguage::CMSConnectLanguage;
use crate::metadata::CMSConnectPersonalization::CMSConnectPersonalization;
use crate::metadata::CMSConnectResourceType::CMSConnectResourceType;
use crate::metadata::CMSConnectionSourceType::CMSConnectionSourceType;
use crate::metadata::CMSConnectionStatus::CMSConnectionStatus;
use crate::metadata::CMSSourceConnectionType::CMSSourceConnectionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CMSConnectSource  {
	#[serde(rename = "cmsConnectAsset")]
	pub cms_connect_asset: Option<Vec<CMSConnectAsset>>,
	#[serde(rename = "cmsConnectLanguage")]
	pub cms_connect_language: Option<Vec<CMSConnectLanguage>>,
	#[serde(rename = "cmsConnectPersonalization")]
	pub cms_connect_personalization: Option<CMSConnectPersonalization>,
	#[serde(rename = "cmsConnectResourceType")]
	pub cms_connect_resource_type: Option<Vec<CMSConnectResourceType>>,
	#[serde(rename = "connectionType")]
	pub connection_type: CMSSourceConnectionType,
	#[serde(rename = "cssScope")]
	pub css_scope: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "languageEnabled")]
	pub language_enabled: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "namedCredential")]
	pub named_credential: Option<String>,
	#[serde(rename = "personalizationEnabled")]
	pub personalization_enabled: Option<String>,
	#[serde(rename = "rootPath")]
	pub root_path: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
	#[serde(rename = "status")]
	pub status: CMSConnectionStatus,
	#[serde(rename = "type")]
	pub _type: CMSConnectionSourceType,
	#[serde(rename = "websiteUrl")]
	pub website_url: Option<String>,
}
