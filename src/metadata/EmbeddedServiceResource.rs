use crate::metadata::EmbeddedServiceResourceType::EmbeddedServiceResourceType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceResource  {
	#[serde(rename = "resource")]
	pub resource: String,
	#[serde(rename = "resourceType")]
	pub resource_type: EmbeddedServiceResourceType,
}
