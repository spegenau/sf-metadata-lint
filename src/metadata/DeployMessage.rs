use crate::metadata::DeployProblemType::DeployProblemType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeployMessage  {
	#[serde(rename = "changed")]
	pub changed: bool,
	#[serde(rename = "columnNumber")]
	pub column_number: Option<i32>,
	#[serde(rename = "componentType")]
	pub component_type: Option<String>,
	#[serde(rename = "created")]
	pub created: bool,
	#[serde(rename = "createdDate")]
	pub created_date: String,
	#[serde(rename = "deleted")]
	pub deleted: bool,
	#[serde(rename = "fileName")]
	pub file_name: String,
	#[serde(rename = "fullName")]
	pub full_name: String,
	#[serde(rename = "id")]
	pub id: Option<String>,
	#[serde(rename = "lineNumber")]
	pub line_number: Option<i32>,
	#[serde(rename = "problem")]
	pub problem: Option<String>,
	#[serde(rename = "problemType")]
	pub problem_type: Option<DeployProblemType>,
	#[serde(rename = "success")]
	pub success: bool,
}
