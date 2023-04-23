use crate::metadata::APIAccessLevel::APIAccessLevel;
use crate::metadata::PackageTypeMembers::PackageTypeMembers;
use crate::metadata::ProfileObjectPermissions::ProfileObjectPermissions;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Package  {
	#[serde(rename = "apiAccessLevel")]
	pub api_access_level: Option<APIAccessLevel>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "namespacePrefix")]
	pub namespace_prefix: Option<String>,
	#[serde(rename = "objectPermissions")]
	pub object_permissions: Option<Vec<ProfileObjectPermissions>>,
	#[serde(rename = "packageType")]
	pub package_type: Option<String>,
	#[serde(rename = "postInstallClass")]
	pub post_install_class: Option<String>,
	#[serde(rename = "setupWeblink")]
	pub setup_weblink: Option<String>,
	#[serde(rename = "types")]
	pub types: Option<Vec<PackageTypeMembers>>,
	#[serde(rename = "uninstallClass")]
	pub uninstall_class: Option<String>,
	#[serde(rename = "version")]
	pub version: String,
}
