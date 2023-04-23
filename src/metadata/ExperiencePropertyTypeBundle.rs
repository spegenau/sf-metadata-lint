use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExperiencePropertyTypeBundle  {
	#[serde(rename = "defaultDesignConfigMCTBody")]
	pub default_design_config_mct_body: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "schemaMCTBody")]
	pub schema_mct_body: String,
}
