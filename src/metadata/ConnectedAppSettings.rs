use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppSettings  {
	#[serde(rename = "enableAdminApprovedAppsOnly")]
	pub enable_admin_approved_apps_only: Option<bool>,
	#[serde(rename = "enableAdminApprovedAppsOnlyForExternalUser")]
	pub enable_admin_approved_apps_only_for_external_user: Option<bool>,
	#[serde(rename = "enableSkipUserProvisioningWizardWelcomePage")]
	pub enable_skip_user_provisioning_wizard_welcome_page: Option<bool>,
}
