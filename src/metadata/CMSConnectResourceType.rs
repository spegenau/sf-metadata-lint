use crate::metadata::CMSConnectResourceDefinition::CMSConnectResourceDefinition;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CMSConnectResourceType  {
	#[serde(rename = "cmsConnectResourceDefinition")]
	pub cms_connect_resource_definition: Option<Vec<CMSConnectResourceDefinition>>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "resourceType")]
	pub resource_type: String,
}
