use crate::metadata::MonitoredEvents::MonitoredEvents;
use crate::metadata::TransactionSecurityAction::TransactionSecurityAction;
use crate::metadata::TransactionSecurityEventName::TransactionSecurityEventName;
use crate::metadata::TxnSecurityPolicyType::TxnSecurityPolicyType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TransactionSecurityPolicy  {
	#[serde(rename = "action")]
	pub action: TransactionSecurityAction,
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "apexClass")]
	pub apex_class: Option<String>,
	#[serde(rename = "blockMessage")]
	pub block_message: Option<String>,
	#[serde(rename = "customEmailContent")]
	pub custom_email_content: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: Option<String>,
	#[serde(rename = "eventName")]
	pub event_name: Option<TransactionSecurityEventName>,
	#[serde(rename = "eventType")]
	pub event_type: Option<MonitoredEvents>,
	#[serde(rename = "executionUser")]
	pub execution_user: Option<String>,
	#[serde(rename = "flow")]
	pub flow: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "resourceName")]
	pub resource_name: Option<String>,
	#[serde(rename = "type")]
	pub _type: Option<TxnSecurityPolicyType>,
}
