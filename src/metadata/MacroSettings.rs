use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MacroSettings  {
	#[serde(rename = "contextualMacroFiltering")]
	pub contextual_macro_filtering: Option<bool>,
	#[serde(rename = "enableAdvancedSearch")]
	pub enable_advanced_search: Option<bool>,
	#[serde(rename = "macrosInFolders")]
	pub macros_in_folders: Option<bool>,
}
