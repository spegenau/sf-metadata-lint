use crate::metadata::CustomShortcut::CustomShortcut;
use crate::metadata::DefaultShortcut::DefaultShortcut;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KeyboardShortcuts  {
	#[serde(rename = "customShortcuts")]
	pub custom_shortcuts: Option<Vec<CustomShortcut>>,
	#[serde(rename = "defaultShortcuts")]
	pub default_shortcuts: Option<Vec<DefaultShortcut>>,
}
