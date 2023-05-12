use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppAnalyticsSettings  {
	#[serde(rename = "enableSimulationMode")]
	pub enable_simulation_mode: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
