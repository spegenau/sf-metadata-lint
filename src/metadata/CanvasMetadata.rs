use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CanvasMetadata  {
	#[serde(rename = "accessMethod")]
	pub access_method: String,
	#[serde(rename = "canvasOptions")]
	pub canvas_options: Option<String>,
	#[serde(rename = "canvasUrl")]
	pub canvas_url: String,
	#[serde(rename = "lifecycleClass")]
	pub lifecycle_class: Option<String>,
	#[serde(rename = "locationOptions")]
	pub location_options: Option<String>,
	#[serde(rename = "samlInitiationMethod")]
	pub saml_initiation_method: Option<String>,
}
