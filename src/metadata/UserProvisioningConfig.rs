use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UserProvisioningConfig  {
	#[serde(rename = "approvalRequired")]
	pub approval_required: Option<String>,
	#[serde(rename = "connectedApp")]
	pub connected_app: String,
	#[serde(rename = "enabled")]
	pub enabled: Option<bool>,
	#[serde(rename = "enabledOperations")]
	pub enabled_operations: Option<String>,
	#[serde(rename = "flow")]
	pub flow: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "namedCredential")]
	pub named_credential: Option<String>,
	#[serde(rename = "notes")]
	pub notes: Option<String>,
	#[serde(rename = "onUpdateAttributes")]
	pub on_update_attributes: Option<String>,
	#[serde(rename = "reconFilter")]
	pub recon_filter: Option<String>,
	#[serde(rename = "userAccountMapping")]
	pub user_account_mapping: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
