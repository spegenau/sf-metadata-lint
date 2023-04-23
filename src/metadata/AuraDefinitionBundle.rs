use crate::metadata::AuraBundleType::AuraBundleType;
use crate::metadata::AuraDefinitions::AuraDefinitions;
use crate::metadata::PackageVersion::PackageVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AuraDefinitionBundle  {
	#[serde(rename = "SVGContent")]
	pub svg_content: Option<String>,
	#[serde(rename = "apiVersion")]
	pub api_version: Option<f32>,
	#[serde(rename = "auraDefinitions")]
	pub aura_definitions: Option<AuraDefinitions>,
	#[serde(rename = "controllerContent")]
	pub controller_content: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "designContent")]
	pub design_content: Option<String>,
	#[serde(rename = "documentationContent")]
	pub documentation_content: Option<String>,
	#[serde(rename = "helperContent")]
	pub helper_content: Option<String>,
	#[serde(rename = "markup")]
	pub markup: Option<String>,
	#[serde(rename = "modelContent")]
	pub model_content: Option<String>,
	#[serde(rename = "packageVersions")]
	pub package_versions: Option<Vec<PackageVersion>>,
	#[serde(rename = "rendererContent")]
	pub renderer_content: Option<String>,
	#[serde(rename = "styleContent")]
	pub style_content: Option<String>,
	#[serde(rename = "testsuiteContent")]
	pub testsuite_content: Option<String>,
	#[serde(rename = "type")]
	pub _type: Option<AuraBundleType>,
}
