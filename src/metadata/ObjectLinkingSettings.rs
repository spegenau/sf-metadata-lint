use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ObjectLinkingSettings  {
	#[serde(rename = "enableObjectLinking")]
	pub enable_object_linking: Option<bool>,
}
