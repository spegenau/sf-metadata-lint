use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdentityVerificationFieldTranslation  {
	#[serde(rename = "customFieldLabel")]
	pub custom_field_label: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
}
