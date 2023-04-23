use crate::metadata::NavigationMenuItemBranding::NavigationMenuItemBranding;
use crate::metadata::NavigationSubMenu::NavigationSubMenu;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NavigationMenuItem  {
	#[serde(rename = "defaultListViewId")]
	pub default_list_view_id: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "menuItemBranding")]
	pub menu_item_branding: Option<NavigationMenuItemBranding>,
	#[serde(rename = "position")]
	pub position: i32,
	#[serde(rename = "publiclyAvailable")]
	pub publicly_available: Option<bool>,
	#[serde(rename = "subMenu")]
	pub sub_menu: Option<NavigationSubMenu>,
	#[serde(rename = "target")]
	pub target: Option<String>,
	#[serde(rename = "targetPreference")]
	pub target_preference: Option<String>,
	#[serde(rename = "type")]
	pub _type: String,
}
