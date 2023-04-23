use crate::metadata::CanvasMetadata::CanvasMetadata;
use crate::metadata::ConnectedAppAttribute::ConnectedAppAttribute;
use crate::metadata::ConnectedAppCanvasConfig::ConnectedAppCanvasConfig;
use crate::metadata::ConnectedAppIpRange::ConnectedAppIpRange;
use crate::metadata::ConnectedAppMobileDetailConfig::ConnectedAppMobileDetailConfig;
use crate::metadata::ConnectedAppOauthConfig::ConnectedAppOauthConfig;
use crate::metadata::ConnectedAppOauthPolicy::ConnectedAppOauthPolicy;
use crate::metadata::ConnectedAppSamlConfig::ConnectedAppSamlConfig;
use crate::metadata::ConnectedAppSessionPolicy::ConnectedAppSessionPolicy;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedApp  {
	#[serde(rename = "attributes")]
	pub attributes: Option<Vec<ConnectedAppAttribute>>,
	#[serde(rename = "canvas")]
	pub canvas: Option<CanvasMetadata>,
	#[serde(rename = "canvasConfig")]
	pub canvas_config: Option<ConnectedAppCanvasConfig>,
	#[serde(rename = "contactEmail")]
	pub contact_email: String,
	#[serde(rename = "contactPhone")]
	pub contact_phone: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "iconUrl")]
	pub icon_url: Option<String>,
	#[serde(rename = "infoUrl")]
	pub info_url: Option<String>,
	#[serde(rename = "ipRanges")]
	pub ip_ranges: Option<Vec<ConnectedAppIpRange>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "logoUrl")]
	pub logo_url: Option<String>,
	#[serde(rename = "mobileAppConfig")]
	pub mobile_app_config: Option<ConnectedAppMobileDetailConfig>,
	#[serde(rename = "mobileStartUrl")]
	pub mobile_start_url: Option<String>,
	#[serde(rename = "oauthConfig")]
	pub oauth_config: Option<ConnectedAppOauthConfig>,
	#[serde(rename = "oauthPolicy")]
	pub oauth_policy: Option<ConnectedAppOauthPolicy>,
	#[serde(rename = "permissionSetName")]
	pub permission_set_name: Option<Vec<String>>,
	#[serde(rename = "plugin")]
	pub plugin: Option<String>,
	#[serde(rename = "pluginExecutionUser")]
	pub plugin_execution_user: Option<String>,
	#[serde(rename = "profileName")]
	pub profile_name: Option<Vec<String>>,
	#[serde(rename = "samlConfig")]
	pub saml_config: Option<ConnectedAppSamlConfig>,
	#[serde(rename = "sessionPolicy")]
	pub session_policy: Option<ConnectedAppSessionPolicy>,
	#[serde(rename = "startUrl")]
	pub start_url: Option<String>,
}
