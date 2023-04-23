use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PrivacySettings  {
	#[serde(rename = "authorizationCaptureBrowser")]
	pub authorization_capture_browser: Option<bool>,
	#[serde(rename = "authorizationCaptureEmail")]
	pub authorization_capture_email: Option<bool>,
	#[serde(rename = "authorizationCaptureIp")]
	pub authorization_capture_ip: Option<bool>,
	#[serde(rename = "authorizationCaptureLocation")]
	pub authorization_capture_location: Option<bool>,
	#[serde(rename = "authorizationCustomSharing")]
	pub authorization_custom_sharing: Option<bool>,
	#[serde(rename = "authorizationLockingAndVersioning")]
	pub authorization_locking_and_versioning: Option<bool>,
	#[serde(rename = "enableConfigurableUserPIIActive")]
	pub enable_configurable_user_pii_active: Option<bool>,
	#[serde(rename = "enableConsentAuditTrail")]
	pub enable_consent_audit_trail: Option<bool>,
	#[serde(rename = "enableConsentEventStream")]
	pub enable_consent_event_stream: Option<bool>,
	#[serde(rename = "enableDefaultMetadataValues")]
	pub enable_default_metadata_values: Option<bool>,
}
