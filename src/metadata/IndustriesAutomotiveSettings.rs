use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IndustriesAutomotiveSettings  {
	#[serde(rename = "enableAutomotiveCloud")]
	pub enable_automotive_cloud: Option<bool>,
	#[serde(rename = "enableAutomotiveServiceExcellence")]
	pub enable_automotive_service_excellence: Option<bool>,
}
