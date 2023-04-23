use crate::metadata::FlowComplexLiteralTranslation::FlowComplexLiteralTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowFerovTranslation  {
	#[serde(rename = "complexValues")]
	pub complex_values: Option<Vec<FlowComplexLiteralTranslation>>,
	#[serde(rename = "stringValue")]
	pub string_value: Option<String>,
}
