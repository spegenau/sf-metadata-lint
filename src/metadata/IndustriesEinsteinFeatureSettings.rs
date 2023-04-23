use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IndustriesEinsteinFeatureSettings  {
	#[serde(rename = "documentReaderConfidenceOrgValue")]
	pub document_reader_confidence_org_value: f32,
}
