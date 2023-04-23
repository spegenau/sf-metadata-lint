use crate::metadata::SidebarComponent::SidebarComponent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Container  {
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "isContainerAutoSizeEnabled")]
	pub is_container_auto_size_enabled: bool,
	#[serde(rename = "region")]
	pub region: String,
	#[serde(rename = "sidebarComponents")]
	pub sidebar_components: Option<Vec<SidebarComponent>>,
	#[serde(rename = "style")]
	pub style: String,
	#[serde(rename = "unit")]
	pub unit: String,
	#[serde(rename = "width")]
	pub width: Option<i32>,
}
