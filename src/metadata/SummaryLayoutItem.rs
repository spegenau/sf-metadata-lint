use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SummaryLayoutItem  {
	#[serde(rename = "customLink")]
	pub custom_link: Option<String>,
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "posX")]
	pub pos_x: i32,
	#[serde(rename = "posY")]
	pub pos_y: Option<i32>,
	#[serde(rename = "posZ")]
	pub pos_z: Option<i32>,
}
