use crate::metadata::CustomHelpMenuItem::CustomHelpMenuItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomHelpMenuSection  {
	#[serde(rename = "customHelpMenuItems")]
	pub custom_help_menu_items: Option<Vec<CustomHelpMenuItem>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
