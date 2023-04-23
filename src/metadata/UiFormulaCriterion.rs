use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UiFormulaCriterion  {
	#[serde(rename = "leftValue")]
	pub left_value: String,
	#[serde(rename = "operator")]
	pub operator: String,
	#[serde(rename = "rightValue")]
	pub right_value: Option<String>,
}
