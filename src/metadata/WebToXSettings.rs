use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WebToXSettings  {
	#[serde(rename = "shouldHideRecordInfoInEmail")]
	pub should_hide_record_info_in_email: Option<bool>,
	#[serde(rename = "webToCaseSpamFilter")]
	pub web_to_case_spam_filter: Option<bool>,
	#[serde(rename = "webToLeadSpamFilter")]
	pub web_to_lead_spam_filter: Option<bool>,
}
