use crate::metadata::ReputationBranding::ReputationBranding;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReputationLevel  {
	#[serde(rename = "branding")]
	pub branding: Option<ReputationBranding>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "lowerThreshold")]
	pub lower_threshold: f32,
}
