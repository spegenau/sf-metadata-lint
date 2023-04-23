use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NextAutomatedApprover  {
	#[serde(rename = "useApproverFieldOfRecordOwner")]
	pub use_approver_field_of_record_owner: Option<bool>,
	#[serde(rename = "userHierarchyField")]
	pub user_hierarchy_field: String,
}
