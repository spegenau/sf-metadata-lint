use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DataImportManagementSettings  {
	#[serde(rename = "enableDataConnectorHubspot")]
	pub enable_data_connector_hubspot: Option<bool>,
	#[serde(rename = "enableEasyImport")]
	pub enable_easy_import: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
