use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LookupFilterTranslation  {
	#[serde(rename = "errorMessage")]
	pub error_message: String,
	#[serde(rename = "informationalMessage")]
	pub informational_message: String,
}
