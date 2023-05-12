use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EventSettings  {
	#[serde(rename = "bypassMeteringBlock")]
	pub bypass_metering_block: Option<bool>,
	#[serde(rename = "enableApexLimitEvents")]
	pub enable_apex_limit_events: Option<bool>,
	#[serde(rename = "enableDeleteMonitoringData")]
	pub enable_delete_monitoring_data: Option<bool>,
	#[serde(rename = "enableDynamicStreamingChannel")]
	pub enable_dynamic_streaming_channel: Option<bool>,
	#[serde(rename = "enableEventLogGeneration")]
	pub enable_event_log_generation: Option<bool>,
	#[serde(rename = "enableEventLogWaveIntegration")]
	pub enable_event_log_wave_integration: Option<bool>,
	#[serde(rename = "enableLoginForensics")]
	pub enable_login_forensics: Option<bool>,
	#[serde(rename = "enableStreamingApi")]
	pub enable_streaming_api: Option<bool>,
	#[serde(rename = "enableTerminateOldestSession")]
	pub enable_terminate_oldest_session: Option<bool>,
	#[serde(rename = "enableTransactionSecurityPolicies")]
	pub enable_transaction_security_policies: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
