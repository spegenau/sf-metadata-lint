use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EclairMap  {
	#[serde(rename = "boundingBoxBottom")]
	pub bounding_box_bottom: Option<f32>,
	#[serde(rename = "boundingBoxLeft")]
	pub bounding_box_left: Option<f32>,
	#[serde(rename = "boundingBoxRight")]
	pub bounding_box_right: Option<f32>,
	#[serde(rename = "boundingBoxTop")]
	pub bounding_box_top: Option<f32>,
	#[serde(rename = "mapLabel")]
	pub map_label: Option<String>,
	#[serde(rename = "mapName")]
	pub map_name: String,
	#[serde(rename = "projection")]
	pub projection: String,
}
