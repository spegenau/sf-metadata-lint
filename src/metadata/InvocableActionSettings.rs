use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InvocableActionSettings  {
	#[serde(rename = "isPartialSaveAllowed")]
	pub is_partial_save_allowed: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
