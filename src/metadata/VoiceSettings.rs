use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct VoiceSettings  {
	#[serde(rename = "enableCallDisposition")]
	pub enable_call_disposition: Option<bool>,
	#[serde(rename = "enableConsentReminder")]
	pub enable_consent_reminder: Option<bool>,
	#[serde(rename = "enableDefaultRecording")]
	pub enable_default_recording: Option<bool>,
	#[serde(rename = "enableVoiceCallList")]
	pub enable_voice_call_list: Option<bool>,
	#[serde(rename = "enableVoiceCallRecording")]
	pub enable_voice_call_recording: Option<bool>,
	#[serde(rename = "enableVoiceCoaching")]
	pub enable_voice_coaching: Option<bool>,
	#[serde(rename = "enableVoiceConferencing")]
	pub enable_voice_conferencing: Option<bool>,
	#[serde(rename = "enableVoiceLocalPresence")]
	pub enable_voice_local_presence: Option<bool>,
	#[serde(rename = "enableVoiceMail")]
	pub enable_voice_mail: Option<bool>,
	#[serde(rename = "enableVoiceMailDrop")]
	pub enable_voice_mail_drop: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
