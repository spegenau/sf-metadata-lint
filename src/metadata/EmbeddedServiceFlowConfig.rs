use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceFlowConfig  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
}
