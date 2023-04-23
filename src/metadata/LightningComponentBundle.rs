use crate::metadata::Capabilities::Capabilities;
use crate::metadata::LwcResources::LwcResources;
use crate::metadata::Targets::Targets;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningComponentBundle  {
	#[serde(rename = "apiVersion")]
	pub api_version: Option<f32>,
	#[serde(rename = "capabilities")]
	pub capabilities: Option<Capabilities>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isExplicitImport")]
	pub is_explicit_import: Option<bool>,
	#[serde(rename = "isExposed")]
	pub is_exposed: Option<bool>,
	#[serde(rename = "lwcResources")]
	pub lwc_resources: Option<LwcResources>,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "runtimeNamespace")]
	pub runtime_namespace: Option<String>,
	#[serde(rename = "targetConfigs")]
	pub target_configs: Option<String>,
	#[serde(rename = "targets")]
	pub targets: Option<Targets>,
}
