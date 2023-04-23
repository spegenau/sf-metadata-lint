use crate::metadata::NavigationMenuItem::NavigationMenuItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NavigationSubMenu  {
	#[serde(rename = "navigationMenuItem")]
	pub navigation_menu_item: Option<Vec<NavigationMenuItem>>,
}
