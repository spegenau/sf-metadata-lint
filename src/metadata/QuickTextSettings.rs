use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickTextSettings  {
	#[serde(rename = "hideQuickTextUiInLtng")]
	pub hide_quick_text_ui_in_ltng: Option<bool>,
	#[serde(rename = "lightningQuickTextEnabled")]
	pub lightning_quick_text_enabled: Option<bool>,
	#[serde(rename = "quickTextsInFolders")]
	pub quick_texts_in_folders: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
