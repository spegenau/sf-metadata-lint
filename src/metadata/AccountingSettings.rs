use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountingSettings  {
	#[serde(rename = "enableAccountingSubledger")]
	pub enable_accounting_subledger: Option<bool>,
	#[serde(rename = "enableFinancePeriod")]
	pub enable_finance_period: Option<bool>,
	#[serde(rename = "enablePaymentMethodAdjust")]
	pub enable_payment_method_adjust: Option<bool>,
	#[serde(rename = "enableScheduledJob")]
	pub enable_scheduled_job: Option<bool>,
}
