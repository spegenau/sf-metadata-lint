use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountSettings  {
	#[serde(rename = "enableAccountDiscovery")]
	pub enable_account_discovery: Option<bool>,
	#[serde(rename = "enableAccountHistoryTracking")]
	pub enable_account_history_tracking: Option<bool>,
	#[serde(rename = "enableAccountInsightsInMobile")]
	pub enable_account_insights_in_mobile: Option<bool>,
	#[serde(rename = "enableAccountOwnerReport")]
	pub enable_account_owner_report: Option<bool>,
	#[serde(rename = "enableAccountTeams")]
	pub enable_account_teams: Option<bool>,
	#[serde(rename = "enableContactHistoryTracking")]
	pub enable_contact_history_tracking: Option<bool>,
	#[serde(rename = "enableRelateContactToMultipleAccounts")]
	pub enable_relate_contact_to_multiple_accounts: Option<bool>,
	#[serde(rename = "showViewHierarchyLink")]
	pub show_view_hierarchy_link: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
