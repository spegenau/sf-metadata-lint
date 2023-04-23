use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningExperienceTheme  {
	#[serde(rename = "defaultBrandingSet")]
	pub default_branding_set: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "shouldOverrideLoadingImage")]
	pub should_override_loading_image: Option<bool>,
}
