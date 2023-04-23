use crate::metadata::AccessMapping::AccessMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingSet  {
	#[serde(rename = "accessMappings")]
	pub access_mappings: Option<Vec<AccessMapping>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "profiles")]
	pub profiles: Option<Vec<String>>,
}
