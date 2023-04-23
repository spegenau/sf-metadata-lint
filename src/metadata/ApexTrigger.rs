use crate::metadata::ApexCodeUnitStatus::ApexCodeUnitStatus;
use crate::metadata::PackageVersion::PackageVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexTrigger  {
	#[serde(rename = "apiVersion")]
	pub api_version: f32,
	#[serde(rename = "packageVersions")]
	pub package_versions: Option<Vec<PackageVersion>>,
	#[serde(rename = "status")]
	pub status: ApexCodeUnitStatus,
}
