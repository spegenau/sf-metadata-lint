use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailIntegrationSettings  {
	#[serde(rename = "doesEmailLogAsEmailMessageInOutlook")]
	pub does_email_log_as_email_message_in_outlook: Option<bool>,
	#[serde(rename = "doesGmailStayConnectedToSalesforce")]
	pub does_gmail_stay_connected_to_salesforce: Option<bool>,
	#[serde(rename = "enableContactAndEventSync")]
	pub enable_contact_and_event_sync: Option<bool>,
	#[serde(rename = "enableEmailTrackingInMobile")]
	pub enable_email_tracking_in_mobile: Option<bool>,
	#[serde(rename = "enableEngageForOutlook")]
	pub enable_engage_for_outlook: Option<bool>,
	#[serde(rename = "enableGmailIntegration")]
	pub enable_gmail_integration: Option<bool>,
	#[serde(rename = "enableInboxMobileIntune")]
	pub enable_inbox_mobile_intune: Option<bool>,
	#[serde(rename = "enableOutlookIntegration")]
	pub enable_outlook_integration: Option<bool>,
	#[serde(rename = "enableOutlookMobileIntegration")]
	pub enable_outlook_mobile_integration: Option<bool>,
	#[serde(rename = "enableProductivityFeatures")]
	pub enable_productivity_features: Option<bool>,
	#[serde(rename = "enableSupplementalContactInfoInMobile")]
	pub enable_supplemental_contact_info_in_mobile: Option<bool>,
	#[serde(rename = "isLayoutCustomizationAllowed")]
	pub is_layout_customization_allowed: Option<bool>,
	#[serde(rename = "orgIsSyncingEventsOutbound")]
	pub org_is_syncing_events_outbound: Option<bool>,
	#[serde(rename = "shouldUseTrustedDomainsList")]
	pub should_use_trusted_domains_list: Option<bool>,
}
