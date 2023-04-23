use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SourceTrackingSettings  {
	#[serde(rename = "enableSourceTrackingSandboxes")]
	pub enable_source_tracking_sandboxes: Option<bool>,
}
