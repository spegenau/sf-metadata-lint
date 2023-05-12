use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BranchManagementSettings  {
	#[serde(rename = "associateAccountWithBranch")]
	pub associate_account_with_branch: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
