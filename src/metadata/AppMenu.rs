use crate::metadata::AppMenuItem::AppMenuItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppMenu  {
	#[serde(rename = "appMenuItems")]
	pub app_menu_items: Option<Vec<AppMenuItem>>,
}
