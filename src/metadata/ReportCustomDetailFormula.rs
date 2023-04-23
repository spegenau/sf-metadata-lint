use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportCustomDetailFormula  {
	#[serde(rename = "calculatedFormula")]
	pub calculated_formula: String,
	#[serde(rename = "dataType")]
	pub data_type: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "scale")]
	pub scale: i32,
}
