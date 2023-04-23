use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowInputValidationRule  {
	#[serde(rename = "errorMessage")]
	pub error_message: String,
	#[serde(rename = "formulaExpression")]
	pub formula_expression: String,
}
