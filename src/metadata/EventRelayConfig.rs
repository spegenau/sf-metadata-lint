use crate::metadata::EventRelayAdminState::EventRelayAdminState;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EventRelayConfig  {
	#[serde(rename = "destinationResourceName")]
	pub destination_resource_name: String,
	#[serde(rename = "eventChannel")]
	pub event_channel: String,
	#[serde(rename = "relayOption")]
	pub relay_option: Option<String>,
	#[serde(rename = "state")]
	pub state: Option<EventRelayAdminState>,
}
