use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceFlowConfig  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
