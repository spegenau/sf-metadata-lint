use crate::metadata::ClientAuthMode::ClientAuthMode;
use crate::metadata::ConversationVendorType::ConversationVendorType;
use crate::metadata::ServerAuthMode::ServerAuthMode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationVendorInfo  {
	#[serde(rename = "agentSSOSupported")]
	pub agent_sso_supported: Option<bool>,
	#[serde(rename = "awsAccountKey")]
	pub aws_account_key: Option<String>,
	#[serde(rename = "awsRootEmail")]
	pub aws_root_email: Option<String>,
	#[serde(rename = "awsTenantVersion")]
	pub aws_tenant_version: Option<f32>,
	#[serde(rename = "bridgeComponent")]
	pub bridge_component: Option<String>,
	#[serde(rename = "clientAuthMode")]
	pub client_auth_mode: Option<ClientAuthMode>,
	#[serde(rename = "connectorUrl")]
	pub connector_url: Option<String>,
	#[serde(rename = "customConfig")]
	pub custom_config: Option<String>,
	#[serde(rename = "customLoginUrl")]
	pub custom_login_url: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "einsteinConversationInsightsSupported")]
	pub einstein_conversation_insights_supported: Option<bool>,
	#[serde(rename = "integrationClass")]
	pub integration_class: Option<String>,
	#[serde(rename = "integrationClassName")]
	pub integration_class_name: Option<String>,
	#[serde(rename = "isTaxCompliant")]
	pub is_tax_compliant: Option<bool>,
	#[serde(rename = "keyProvisioningSupported")]
	pub key_provisioning_supported: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "namedCredential")]
	pub named_credential: Option<String>,
	#[serde(rename = "namedCredentialSupported")]
	pub named_credential_supported: Option<bool>,
	#[serde(rename = "partnerContactCenterListSupported")]
	pub partner_contact_center_list_supported: Option<bool>,
	#[serde(rename = "partnerPhoneNumbersSupported")]
	pub partner_phone_numbers_supported: Option<bool>,
	#[serde(rename = "partnerTransferDestinationsSupported")]
	pub partner_transfer_destinations_supported: Option<bool>,
	#[serde(rename = "queueManagementSupported")]
	pub queue_management_supported: Option<bool>,
	#[serde(rename = "serverAuthMode")]
	pub server_auth_mode: Option<ServerAuthMode>,
	#[serde(rename = "telephonySettingsComponent")]
	pub telephony_settings_component: Option<String>,
	#[serde(rename = "universalCallRecordingAccessSupported")]
	pub universal_call_recording_access_supported: Option<bool>,
	#[serde(rename = "userSyncingSupported")]
	pub user_syncing_supported: Option<bool>,
	#[serde(rename = "vendorType")]
	pub vendor_type: Option<ConversationVendorType>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
