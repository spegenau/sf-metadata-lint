use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EssentialsSettings  {
	#[serde(rename = "emailConnectorEnabled")]
	pub email_connector_enabled: Option<bool>,
}
