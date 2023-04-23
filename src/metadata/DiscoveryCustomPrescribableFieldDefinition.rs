use crate::metadata::DiscoveryFilter::DiscoveryFilter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryCustomPrescribableFieldDefinition  {
	#[serde(rename = "filters")]
	pub filters: Option<Vec<DiscoveryFilter>>,
	#[serde(rename = "template")]
	pub template: Option<String>,
}
