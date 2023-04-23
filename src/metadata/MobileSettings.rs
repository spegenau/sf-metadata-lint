use crate::metadata::DashboardMobileSettings::DashboardMobileSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MobileSettings  {
	#[serde(rename = "dashboardMobile")]
	pub dashboard_mobile: Option<DashboardMobileSettings>,
	#[serde(rename = "enableImportContactFromDevice")]
	pub enable_import_contact_from_device: Option<bool>,
	#[serde(rename = "enableOfflineDraftsEnabled")]
	pub enable_offline_drafts_enabled: Option<bool>,
	#[serde(rename = "enablePopulateNameManuallyInToday")]
	pub enable_populate_name_manually_in_today: Option<bool>,
	#[serde(rename = "enableS1EncryptedStoragePref2")]
	pub enable_s_1_encrypted_storage_pref_2: Option<bool>,
	#[serde(rename = "enableS1OfflinePref")]
	pub enable_s_1_offline_pref: Option<bool>,
}
