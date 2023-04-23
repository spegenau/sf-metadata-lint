use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningBoltFeatures  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "order")]
	pub order: i32,
	#[serde(rename = "title")]
	pub title: String,
}
