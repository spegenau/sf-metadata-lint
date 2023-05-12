use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailAdministrationSettings  {
	#[serde(rename = "enableComplianceBcc")]
	pub enable_compliance_bcc: Option<bool>,
	#[serde(rename = "enableEmailConsentManagement")]
	pub enable_email_consent_management: Option<bool>,
	#[serde(rename = "enableEmailSenderIdCompliance")]
	pub enable_email_sender_id_compliance: Option<bool>,
	#[serde(rename = "enableEmailSpfCompliance")]
	pub enable_email_spf_compliance: Option<bool>,
	#[serde(rename = "enableEmailToSalesforce")]
	pub enable_email_to_salesforce: Option<bool>,
	#[serde(rename = "enableEmailTrackingIPBlocklist")]
	pub enable_email_tracking_ip_blocklist: Option<bool>,
	#[serde(rename = "enableEmailWorkflowApproval")]
	pub enable_email_workflow_approval: Option<bool>,
	#[serde(rename = "enableEnhancedEmailEnabled")]
	pub enable_enhanced_email_enabled: Option<bool>,
	#[serde(rename = "enableHandleBouncedEmails")]
	pub enable_handle_bounced_emails: Option<bool>,
	#[serde(rename = "enableHtmlEmail")]
	pub enable_html_email: Option<bool>,
	#[serde(rename = "enableInternationalEmailAddresses")]
	pub enable_international_email_addresses: Option<bool>,
	#[serde(rename = "enableListEmailLogActivities")]
	pub enable_list_email_log_activities: Option<bool>,
	#[serde(rename = "enableResendBouncedEmails")]
	pub enable_resend_bounced_emails: Option<bool>,
	#[serde(rename = "enableRestrictTlsToDomains")]
	pub enable_restrict_tls_to_domains: Option<bool>,
	#[serde(rename = "enableSendThroughGmailPref")]
	pub enable_send_through_gmail_pref: Option<bool>,
	#[serde(rename = "enableSendViaExchangePref")]
	pub enable_send_via_exchange_pref: Option<bool>,
	#[serde(rename = "enableSendViaGmailPref")]
	pub enable_send_via_gmail_pref: Option<bool>,
	#[serde(rename = "enableUseOrgFootersForExtTrans")]
	pub enable_use_org_footers_for_ext_trans: Option<bool>,
	#[serde(rename = "enableVerifyEmailDomainByDkim")]
	pub enable_verify_email_domain_by_dkim: Option<bool>,
	#[serde(rename = "sendEmailsEvenWhenAutomationUpdatesSameRecord")]
	pub send_emails_even_when_automation_updates_same_record: Option<bool>,
	#[serde(rename = "sendMassEmailNotification")]
	pub send_mass_email_notification: Option<bool>,
	#[serde(rename = "sendTextOnlySystemEmails")]
	pub send_text_only_system_emails: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
