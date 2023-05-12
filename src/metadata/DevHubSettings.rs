use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DevHubSettings  {
	#[serde(rename = "devOpsCenterBetaMsa")]
	pub dev_ops_center_beta_msa: Option<bool>,
	#[serde(rename = "enableDevOpsCenter")]
	pub enable_dev_ops_center: Option<bool>,
	#[serde(rename = "enableDevOpsCenterGA")]
	pub enable_dev_ops_center_ga: Option<bool>,
	#[serde(rename = "enablePackaging2")]
	pub enable_packaging_2: Option<bool>,
	#[serde(rename = "enableScratchOrgManagementPref")]
	pub enable_scratch_org_management_pref: Option<bool>,
	#[serde(rename = "enableShapeExportPref")]
	pub enable_shape_export_pref: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
