use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReputationBranding  {
	#[serde(rename = "smallImage")]
	pub small_image: Option<String>,
}
