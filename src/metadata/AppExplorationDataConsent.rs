use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppExplorationDataConsent  {
	#[serde(rename = "applicationName")]
	pub application_name: Option<String>,
	#[serde(rename = "availableObjects")]
	pub available_objects: Option<String>,
	#[serde(rename = "enabledObjects")]
	pub enabled_objects: Option<String>,
	#[serde(rename = "isProjectAccessEnabled")]
	pub is_project_access_enabled: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "projectName")]
	pub project_name: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
