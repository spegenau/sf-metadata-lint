use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DataDotComSettings  {
	#[serde(rename = "enableAccountExportButtonOff")]
	pub enable_account_export_button_off: Option<bool>,
	#[serde(rename = "enableAccountImportButtonOff")]
	pub enable_account_import_button_off: Option<bool>,
	#[serde(rename = "enableAllowDupeContactFromLead")]
	pub enable_allow_dupe_contact_from_lead: Option<bool>,
	#[serde(rename = "enableAllowDupeLeadFromContact")]
	pub enable_allow_dupe_lead_from_contact: Option<bool>,
	#[serde(rename = "enableContactExportButtonOff")]
	pub enable_contact_export_button_off: Option<bool>,
	#[serde(rename = "enableContactImportButtonOff")]
	pub enable_contact_import_button_off: Option<bool>,
	#[serde(rename = "enableDDCSocialKeyEnabled")]
	pub enable_ddc_social_key_enabled: Option<bool>,
	#[serde(rename = "enableDataDotComCleanEnabled")]
	pub enable_data_dot_com_clean_enabled: Option<bool>,
	#[serde(rename = "enableDataDotComOptOutsEnabled")]
	pub enable_data_dot_com_opt_outs_enabled: Option<bool>,
	#[serde(rename = "enableDatacloudAPIEnabled")]
	pub enable_datacloud_api_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
