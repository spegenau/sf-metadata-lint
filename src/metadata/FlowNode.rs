use crate::metadata::FlowElementSubtype::FlowElementSubtype;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowNode  {
	#[serde(rename = "elementSubtype")]
	pub element_subtype: Option<FlowElementSubtype>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "locationX")]
	pub location_x: i32,
	#[serde(rename = "locationY")]
	pub location_y: i32,
}
