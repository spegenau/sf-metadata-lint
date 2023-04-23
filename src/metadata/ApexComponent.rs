use crate::metadata::PackageVersion::PackageVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexComponent  {
	#[serde(rename = "apiVersion")]
	pub api_version: Option<f32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "packageVersions")]
	pub package_versions: Option<Vec<PackageVersion>>,
}
