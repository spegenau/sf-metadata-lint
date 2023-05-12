use crate::metadata::ExtensionPointName::ExtensionPointName;
use crate::metadata::RegistryProviderType::RegistryProviderType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RegisteredExternalService  {
	#[serde(rename = "configUrl")]
	pub config_url: Option<String>,
	#[serde(rename = "documentationUrl")]
	pub documentation_url: Option<String>,
	#[serde(rename = "extensionPointName")]
	pub extension_point_name: Option<ExtensionPointName>,
	#[serde(rename = "externalServiceProvider")]
	pub external_service_provider: String,
	#[serde(rename = "externalServiceProviderType")]
	pub external_service_provider_type: RegistryProviderType,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
