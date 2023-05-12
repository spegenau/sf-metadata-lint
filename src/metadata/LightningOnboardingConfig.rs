use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningOnboardingConfig  {
	#[serde(rename = "collaborationGroup")]
	pub collaboration_group: String,
	#[serde(rename = "customQuestion")]
	pub custom_question: Option<String>,
	#[serde(rename = "feedbackFormDaysFrequency")]
	pub feedback_form_days_frequency: i32,
	#[serde(rename = "isCustom")]
	pub is_custom: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "promptDelayTime")]
	pub prompt_delay_time: i32,
	#[serde(rename = "sendFeedbackToSalesforce")]
	pub send_feedback_to_salesforce: bool,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
