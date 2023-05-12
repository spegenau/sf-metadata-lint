use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SubscriptionManagementSettings  {
	#[serde(rename = "enableConvertNegativeInvoiceLinesToCreditMemoAndApply")]
	pub enable_convert_negative_invoice_lines_to_credit_memo_and_apply: Option<bool>,
	#[serde(rename = "enablePaymentScheduleAutomation")]
	pub enable_payment_schedule_automation: Option<bool>,
	#[serde(rename = "enableRefundAutomation")]
	pub enable_refund_automation: Option<bool>,
	#[serde(rename = "enableSubscriptionManagement")]
	pub enable_subscription_management: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
