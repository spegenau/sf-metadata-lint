use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MutingPermissionSet  {
	#[serde(rename = "label")]
	pub label: String,
}
