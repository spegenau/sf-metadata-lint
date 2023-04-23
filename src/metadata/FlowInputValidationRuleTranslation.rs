use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowInputValidationRuleTranslation  {
	#[serde(rename = "errorMessage")]
	pub error_message: Option<String>,
}
