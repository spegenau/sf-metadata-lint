use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PresenceDeclineReason  {
	#[serde(rename = "label")]
	pub label: String,
}
