use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExperienceBundleSettings  {
	#[serde(rename = "enableExperienceBundleMetadata")]
	pub enable_experience_bundle_metadata: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
