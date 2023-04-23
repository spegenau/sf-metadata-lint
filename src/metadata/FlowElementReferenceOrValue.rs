use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowElementReferenceOrValue  {
	#[serde(rename = "apexValue")]
	pub apex_value: Option<String>,
	#[serde(rename = "booleanValue")]
	pub boolean_value: Option<bool>,
	#[serde(rename = "dateTimeValue")]
	pub date_time_value: Option<String>,
	#[serde(rename = "dateValue")]
	pub date_value: Option<String>,
	#[serde(rename = "elementReference")]
	pub element_reference: Option<String>,
	#[serde(rename = "numberValue")]
	pub number_value: Option<f32>,
	#[serde(rename = "sobjectValue")]
	pub sobject_value: Option<String>,
	#[serde(rename = "stringValue")]
	pub string_value: Option<String>,
}
