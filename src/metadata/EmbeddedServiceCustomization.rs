use crate::metadata::EmbeddedServiceResource::EmbeddedServiceResource;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceCustomization  {
	#[serde(rename = "customizationName")]
	pub customization_name: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "embeddedServiceResources")]
	pub embedded_service_resources: Option<Vec<EmbeddedServiceResource>>,
}
