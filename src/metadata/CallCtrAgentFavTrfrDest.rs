use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CallCtrAgentFavTrfrDest  {
	#[serde(rename = "agent")]
	pub agent: String,
	#[serde(rename = "callCenter")]
	pub call_center: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "transferDestination")]
	pub transfer_destination: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
