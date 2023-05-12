use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContractSettings  {
	#[serde(rename = "autoCalculateEndDate")]
	pub auto_calculate_end_date: Option<bool>,
	#[serde(rename = "autoExpirationDelay")]
	pub auto_expiration_delay: Option<String>,
	#[serde(rename = "autoExpirationRecipient")]
	pub auto_expiration_recipient: Option<String>,
	#[serde(rename = "autoExpireContracts")]
	pub auto_expire_contracts: Option<bool>,
	#[serde(rename = "enableContractHistoryTracking")]
	pub enable_contract_history_tracking: Option<bool>,
	#[serde(rename = "notifyOwnersOnContractExpiration")]
	pub notify_owners_on_contract_expiration: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
