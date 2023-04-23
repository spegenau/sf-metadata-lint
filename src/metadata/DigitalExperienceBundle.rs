use crate::metadata::DigitalExperience::DigitalExperience;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DigitalExperienceBundle  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "spaceResources")]
	pub space_resources: Option<Vec<DigitalExperience>>,
}
