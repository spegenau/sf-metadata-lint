use crate::metadata::AccessMethod::AccessMethod;
use crate::metadata::CanvasLocationOptions::CanvasLocationOptions;
use crate::metadata::CanvasOptions::CanvasOptions;
use crate::metadata::SamlInitiationMethod::SamlInitiationMethod;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppCanvasConfig  {
	#[serde(rename = "accessMethod")]
	pub access_method: AccessMethod,
	#[serde(rename = "canvasUrl")]
	pub canvas_url: String,
	#[serde(rename = "lifecycleClass")]
	pub lifecycle_class: Option<String>,
	#[serde(rename = "locations")]
	pub locations: Option<Vec<CanvasLocationOptions>>,
	#[serde(rename = "options")]
	pub options: Option<Vec<CanvasOptions>>,
	#[serde(rename = "samlInitiationMethod")]
	pub saml_initiation_method: Option<SamlInitiationMethod>,
}
