use crate::metadata::NavigationMenuItem::NavigationMenuItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NavigationMenu  {
	#[serde(rename = "container")]
	pub container: String,
	#[serde(rename = "containerType")]
	pub container_type: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "navigationMenuItem")]
	pub navigation_menu_item: Option<Vec<NavigationMenuItem>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
