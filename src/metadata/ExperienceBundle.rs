use crate::metadata::ExperienceResources::ExperienceResources;
use crate::metadata::SiteType::SiteType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExperienceBundle  {
	#[serde(rename = "experienceResources")]
	pub experience_resources: Option<ExperienceResources>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "type")]
	pub _type: SiteType,
	#[serde(rename = "urlPathPrefix")]
	pub url_path_prefix: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
