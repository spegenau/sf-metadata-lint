use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EinsteinDocumentCaptureSettings  {
	#[serde(rename = "enableEinsteinDocumentReader")]
	pub enable_einstein_document_reader: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
