use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MfgServiceConsoleSettings  {
	#[serde(rename = "enableMfgServiceConsole")]
	pub enable_mfg_service_console: Option<bool>,
}
