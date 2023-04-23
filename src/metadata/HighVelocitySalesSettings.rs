use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct HighVelocitySalesSettings  {
	#[serde(rename = "enableACAutoSendEmail")]
	pub enable_ac_auto_send_email: Option<bool>,
	#[serde(rename = "enableACChangeTargetAssignee")]
	pub enable_ac_change_target_assignee: Option<bool>,
	#[serde(rename = "enableACSkipWeekends")]
	pub enable_ac_skip_weekends: Option<bool>,
	#[serde(rename = "enableCadenceVariantTestingPref")]
	pub enable_cadence_variant_testing_pref: Option<bool>,
	#[serde(rename = "enableChgTgtAssigneeUsrPermPref")]
	pub enable_chg_tgt_assignee_usr_perm_pref: Option<bool>,
	#[serde(rename = "enableDispositionCategory")]
	pub enable_disposition_category: Option<bool>,
	#[serde(rename = "enableEngagementWaveAnalyticsPref")]
	pub enable_engagement_wave_analytics_pref: Option<bool>,
	#[serde(rename = "enableHighVelocitySales")]
	pub enable_high_velocity_sales: Option<bool>,
	#[serde(rename = "enableHighVelocitySalesSetup")]
	pub enable_high_velocity_sales_setup: Option<bool>,
	#[serde(rename = "enableInvoiceAttributionPref")]
	pub enable_invoice_attribution_pref: Option<bool>,
	#[serde(rename = "enableLogACallForCTIPref")]
	pub enable_log_a_call_for_cti_pref: Option<bool>,
	#[serde(rename = "enableLogTasksForLinkedInPref")]
	pub enable_log_tasks_for_linked_in_pref: Option<bool>,
	#[serde(rename = "enableMultipleCadencesPref")]
	pub enable_multiple_cadences_pref: Option<bool>,
	#[serde(rename = "enableOpportunityAttributionPermPref")]
	pub enable_opportunity_attribution_perm_pref: Option<bool>,
	#[serde(rename = "enableQuickCadenceAutoSendEmail")]
	pub enable_quick_cadence_auto_send_email: Option<bool>,
	#[serde(rename = "enableTaskLoggingPref")]
	pub enable_task_logging_pref: Option<bool>,
}
