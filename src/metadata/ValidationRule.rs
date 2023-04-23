use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValidationRule  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "errorConditionFormula")]
	pub error_condition_formula: String,
	#[serde(rename = "errorDisplayField")]
	pub error_display_field: Option<String>,
	#[serde(rename = "errorMessage")]
	pub error_message: String,
}
