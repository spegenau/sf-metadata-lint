use crate::metadata::NavigationMenuItem::NavigationMenuItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NavigationLinkSet  {
	#[serde(rename = "navigationMenuItem")]
	pub navigation_menu_item: Option<Vec<NavigationMenuItem>>,
}
