use crate::metadata::Package::Package;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RetrieveRequest  {
	#[serde(rename = "apiVersion")]
	pub api_version: f32,
	#[serde(rename = "packageNames")]
	pub package_names: Option<Vec<String>>,
	#[serde(rename = "singlePackage")]
	pub single_package: bool,
	#[serde(rename = "specificFiles")]
	pub specific_files: Option<Vec<String>>,
	#[serde(rename = "unpackaged")]
	pub unpackaged: Option<Package>,
}
