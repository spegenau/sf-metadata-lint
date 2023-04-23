use crate::metadata::PackageVersion::PackageVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexPage  {
	#[serde(rename = "apiVersion")]
	pub api_version: f32,
	#[serde(rename = "availableInTouch")]
	pub available_in_touch: Option<bool>,
	#[serde(rename = "confirmationTokenRequired")]
	pub confirmation_token_required: Option<bool>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "packageVersions")]
	pub package_versions: Option<Vec<PackageVersion>>,
}
