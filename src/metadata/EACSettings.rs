use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EACSettings  {
	#[serde(rename = "addRcCompToFlexiPages")]
	pub add_rc_comp_to_flexi_pages: Option<bool>,
	#[serde(rename = "autoPopulateGoogleMeetLinks")]
	pub auto_populate_google_meet_links: Option<bool>,
	#[serde(rename = "automatedEmailFilter")]
	pub automated_email_filter: Option<bool>,
	#[serde(rename = "enableActivityAnalyticsPref")]
	pub enable_activity_analytics_pref: Option<bool>,
	#[serde(rename = "enableActivityCapture")]
	pub enable_activity_capture: Option<bool>,
	#[serde(rename = "enableActivityMetrics")]
	pub enable_activity_metrics: Option<bool>,
	#[serde(rename = "enableActivitySyncEngine")]
	pub enable_activity_sync_engine: Option<bool>,
	#[serde(rename = "enableEACForEveryonePref")]
	pub enable_eac_for_everyone_pref: Option<bool>,
	#[serde(rename = "enableEnforceEacSharingPref")]
	pub enable_enforce_eac_sharing_pref: Option<bool>,
	#[serde(rename = "enableInboxActivitySharing")]
	pub enable_inbox_activity_sharing: Option<bool>,
	#[serde(rename = "enableInsightsInTimeline")]
	pub enable_insights_in_timeline: Option<bool>,
	#[serde(rename = "enableInsightsInTimelineEacStd")]
	pub enable_insights_in_timeline_eac_std: Option<bool>,
	#[serde(rename = "provisionProductivityFeatures")]
	pub provision_productivity_features: Option<bool>,
	#[serde(rename = "salesforceEventsOnlyPref")]
	pub salesforce_events_only_pref: Option<bool>,
	#[serde(rename = "sensitiveEmailFilter")]
	pub sensitive_email_filter: Option<bool>,
	#[serde(rename = "syncInternalEvents")]
	pub sync_internal_events: Option<bool>,
}
