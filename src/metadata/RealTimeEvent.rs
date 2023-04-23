use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RealTimeEvent  {
	#[serde(rename = "entityName")]
	pub entity_name: String,
	#[serde(rename = "isEnabled")]
	pub is_enabled: bool,
}
