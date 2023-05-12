use crate::metadata::FlowTestPoint::FlowTestPoint;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTest  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "flowApiName")]
	pub flow_api_name: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "testPoints")]
	pub test_points: Option<Vec<FlowTestPoint>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
