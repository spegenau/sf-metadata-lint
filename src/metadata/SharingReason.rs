use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingReason  {
	#[serde(rename = "label")]
	pub label: String,
}
