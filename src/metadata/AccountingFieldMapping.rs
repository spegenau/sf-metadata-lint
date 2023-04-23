use crate::metadata::MappingBehaviorType::MappingBehaviorType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountingFieldMapping  {
	#[serde(rename = "isForAllocationType")]
	pub is_for_allocation_type: Option<bool>,
	#[serde(rename = "isForPaymentType")]
	pub is_for_payment_type: Option<bool>,
	#[serde(rename = "isForTransactionType")]
	pub is_for_transaction_type: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "mappingBehavior")]
	pub mapping_behavior: MappingBehaviorType,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sourceField")]
	pub source_field: Option<String>,
	#[serde(rename = "targetField")]
	pub target_field: String,
}
