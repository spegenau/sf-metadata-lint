use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformEncryptionSettings  {
	#[serde(rename = "canEncryptManagedPackageFields")]
	pub can_encrypt_managed_package_fields: Option<bool>,
	#[serde(rename = "enableDeterministicEncryption")]
	pub enable_deterministic_encryption: Option<bool>,
	#[serde(rename = "enableEncryptFieldHistory")]
	pub enable_encrypt_field_history: Option<bool>,
	#[serde(rename = "enableEncryptionSearchEnabled")]
	pub enable_encryption_search_enabled: Option<bool>,
	#[serde(rename = "enableEventBusEncryption")]
	pub enable_event_bus_encryption: Option<bool>,
	#[serde(rename = "isMEKForEncryptionRequired")]
	pub is_mek_for_encryption_required: Option<bool>,
	#[serde(rename = "isUseHighAssuranceKeysRequired")]
	pub is_use_high_assurance_keys_required: Option<bool>,
}
