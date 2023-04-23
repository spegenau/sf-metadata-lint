use crate::metadata::ExperienceResource::ExperienceResource;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExperienceResources  {
	#[serde(rename = "experienceResource")]
	pub experience_resource: Option<Vec<ExperienceResource>>,
}
