use crate::metadata::ApplicationSourceType::ApplicationSourceType;
use crate::metadata::ServiceAISetupDefStatus::ServiceAISetupDefStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceAISetupDefinition  {
	#[serde(rename = "appSourceType")]
	pub app_source_type: ApplicationSourceType,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "setupStatus")]
	pub setup_status: ServiceAISetupDefStatus,
	#[serde(rename = "supportedLanguages")]
	pub supported_languages: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
