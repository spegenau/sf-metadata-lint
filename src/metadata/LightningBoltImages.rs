use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningBoltImages  {
	#[serde(rename = "image")]
	pub image: String,
	#[serde(rename = "order")]
	pub order: i32,
}
