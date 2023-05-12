use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MeetingsSettings  {
	#[serde(rename = "enableSalesforceMeetings")]
	pub enable_salesforce_meetings: Option<bool>,
	#[serde(rename = "enableSalesforceMeetingsSyncCheck")]
	pub enable_salesforce_meetings_sync_check: Option<bool>,
	#[serde(rename = "enableZoomVideoConference")]
	pub enable_zoom_video_conference: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
