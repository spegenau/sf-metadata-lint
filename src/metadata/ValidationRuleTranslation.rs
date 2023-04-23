use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValidationRuleTranslation  {
	#[serde(rename = "errorMessage")]
	pub error_message: String,
	#[serde(rename = "name")]
	pub name: String,
}
