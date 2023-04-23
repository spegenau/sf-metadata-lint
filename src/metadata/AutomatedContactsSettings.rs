use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AutomatedContactsSettings  {
	#[serde(rename = "enableAddContactAutomatically")]
	pub enable_add_contact_automatically: Option<bool>,
	#[serde(rename = "enableAddContactRoleAutomatically")]
	pub enable_add_contact_role_automatically: Option<bool>,
	#[serde(rename = "enableAddContactRoleWithSuggestion")]
	pub enable_add_contact_role_with_suggestion: Option<bool>,
	#[serde(rename = "enableAddContactWithSuggestion")]
	pub enable_add_contact_with_suggestion: Option<bool>,
}
