use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountSharingRuleSettings  {
	#[serde(rename = "caseAccessLevel")]
	pub case_access_level: String,
	#[serde(rename = "contactAccessLevel")]
	pub contact_access_level: String,
	#[serde(rename = "opportunityAccessLevel")]
	pub opportunity_access_level: String,
}
