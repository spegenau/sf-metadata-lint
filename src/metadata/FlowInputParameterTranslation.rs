use crate::metadata::FlowFerovTranslation::FlowFerovTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowInputParameterTranslation  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: FlowFerovTranslation,
}
